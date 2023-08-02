use std::collections::HashMap;

use re_types::{archetypes::Arrows3D, components, Archetype as _};

#[test]
fn roundtrip() {
    let expected = Arrows3D {
        arrows: vec![
            components::Arrow3D::new([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]), //
            components::Arrow3D::new([10.0, 20.0, 30.0], [40.0, 50.0, 60.0]), //
        ],
        radii: Some(vec![
            components::Radius(1.0), //
            components::Radius(10.0),
        ]),
        colors: Some(vec![
            components::Color::from_unmultiplied_rgba(0xAA, 0x00, 0x00, 0xCC), //
            components::Color::from_unmultiplied_rgba(0x00, 0xBB, 0x00, 0xDD),
        ]),
        labels: Some(vec![
            components::Label("hello".to_owned()),  //
            components::Label("friend".to_owned()), //
        ]),
        class_ids: Some(vec![
            components::ClassId(126), //
            components::ClassId(127), //
        ]),
        instance_keys: Some(vec![
            components::InstanceKey(u64::MAX - 1), //
            components::InstanceKey(u64::MAX),
        ]),
    };

    let arch = Arrows3D::new([
        components::Arrow3D::new([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]), //
        components::Arrow3D::new([10.0, 20.0, 30.0], [40.0, 50.0, 60.0]), //
    ])
    .with_radii([1.0, 10.0])
    .with_colors([0xAA0000CC, 0x00BB00DD])
    .with_labels(["hello", "friend"])
    .with_class_ids([126, 127])
    .with_instance_keys([u64::MAX - 1, u64::MAX]);
    similar_asserts::assert_eq!(expected, arch);

    let expected_extensions: HashMap<_, _> = [
        ("arrows", vec!["rerun.components.Arrow3D"]),
        ("radii", vec!["rerun.components.Radius"]),
        ("colors", vec!["rerun.components.Color"]),
        ("labels", vec!["rerun.components.Label"]),
        ("class_ids", vec!["rerun.components.ClassId"]),
        ("instance_keys", vec!["rerun.components.InstanceKey"]),
    ]
    .into();

    eprintln!("arch = {arch:#?}");
    let serialized = arch.to_arrow();
    for (field, array) in &serialized {
        // NOTE: Keep those around please, very useful when debugging.
        // eprintln!("field = {field:#?}");
        // eprintln!("array = {array:#?}");
        eprintln!("{} = {array:#?}", field.name);

        // TODO(cmc): Re-enable extensions and these assertions once `arrow2-convert`
        // has been fully replaced.
        if false {
            util::assert_extensions(
                &**array,
                expected_extensions[field.name.as_str()].as_slice(),
            );
        }
    }

    let deserialized = Arrows3D::from_arrow(serialized);
    similar_asserts::assert_eq!(expected, deserialized);
}

mod util;
