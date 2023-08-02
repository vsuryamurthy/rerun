// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// An arrow in 3D space.
#[derive(Clone, Debug, Default, Copy, PartialEq, PartialOrd)]
pub struct Arrow3D {
    pub origin: crate::datatypes::Vec3D,
    pub vector: crate::datatypes::Vec3D,
}

impl<'a> From<Arrow3D> for ::std::borrow::Cow<'a, Arrow3D> {
    #[inline]
    fn from(value: Arrow3D) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a Arrow3D> for ::std::borrow::Cow<'a, Arrow3D> {
    #[inline]
    fn from(value: &'a Arrow3D) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for Arrow3D {
    type Name = crate::DatatypeName;
    type Item<'a> = Option<Self>;
    type Iter<'a> = Box<dyn Iterator<Item = Self::Item<'a>> + 'a>;
    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.Arrow3D".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Struct(vec![
            Field {
                name: "origin".to_owned(),
                data_type: <crate::datatypes::Vec3D>::to_arrow_datatype(),
                is_nullable: false,
                metadata: [].into(),
            },
            Field {
                name: "vector".to_owned(),
                data_type: <crate::datatypes::Vec3D>::to_arrow_datatype(),
                is_nullable: false,
                metadata: [].into(),
            },
        ])
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
        extension_wrapper: Option<&str>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                (if let Some(ext) = extension_wrapper {
                    DataType::Extension(
                        ext.to_owned(),
                        Box::new(<crate::datatypes::Arrow3D>::to_arrow_datatype()),
                        None,
                    )
                } else {
                    <crate::datatypes::Arrow3D>::to_arrow_datatype()
                })
                .to_logical_type()
                .clone(),
                vec![
                    {
                        let (somes, origin): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { origin, .. } = &**datum;
                                    origin.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let origin_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let origin_inner_data: Vec<_> = origin
                                .iter()
                                .map(|datum| {
                                    datum
                                        .map(|datum| {
                                            let crate::datatypes::Vec3D(data0) = datum;
                                            data0
                                        })
                                        .unwrap_or_default()
                                })
                                .flatten()
                                .map(Some)
                                .collect();
                            let origin_inner_bitmap: Option<::arrow2::bitmap::Bitmap> = None;
                            FixedSizeListArray::new(
                                {
                                    _ = extension_wrapper;
                                    DataType::FixedSizeList(
                                        Box::new(Field {
                                            name: "item".to_owned(),
                                            data_type: DataType::Float32,
                                            is_nullable: false,
                                            metadata: [].into(),
                                        }),
                                        3usize,
                                    )
                                    .to_logical_type()
                                    .clone()
                                },
                                PrimitiveArray::new(
                                    {
                                        _ = extension_wrapper;
                                        DataType::Float32.to_logical_type().clone()
                                    },
                                    origin_inner_data
                                        .into_iter()
                                        .map(|v| v.unwrap_or_default())
                                        .collect(),
                                    origin_inner_bitmap,
                                )
                                .boxed(),
                                origin_bitmap,
                            )
                            .boxed()
                        }
                    },
                    {
                        let (somes, vector): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { vector, .. } = &**datum;
                                    vector.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let vector_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let vector_inner_data: Vec<_> = vector
                                .iter()
                                .map(|datum| {
                                    datum
                                        .map(|datum| {
                                            let crate::datatypes::Vec3D(data0) = datum;
                                            data0
                                        })
                                        .unwrap_or_default()
                                })
                                .flatten()
                                .map(Some)
                                .collect();
                            let vector_inner_bitmap: Option<::arrow2::bitmap::Bitmap> = None;
                            FixedSizeListArray::new(
                                {
                                    _ = extension_wrapper;
                                    DataType::FixedSizeList(
                                        Box::new(Field {
                                            name: "item".to_owned(),
                                            data_type: DataType::Float32,
                                            is_nullable: false,
                                            metadata: [].into(),
                                        }),
                                        3usize,
                                    )
                                    .to_logical_type()
                                    .clone()
                                },
                                PrimitiveArray::new(
                                    {
                                        _ = extension_wrapper;
                                        DataType::Float32.to_logical_type().clone()
                                    },
                                    vector_inner_data
                                        .into_iter()
                                        .map(|v| v.unwrap_or_default())
                                        .collect(),
                                    vector_inner_bitmap,
                                )
                                .boxed(),
                                vector_bitmap,
                            )
                            .boxed()
                        }
                    },
                ],
                bitmap,
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let data = data
                .as_any()
                .downcast_ref::<::arrow2::array::StructArray>()
                .ok_or_else(|| crate::DeserializationError::DatatypeMismatch {
                    expected: data.data_type().clone(),
                    got: data.data_type().clone(),
                    backtrace: ::backtrace::Backtrace::new_unresolved(),
                })
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.datatypes.Arrow3D".into(),
                    source: Box::new(err),
                })?;
            if data.is_empty() {
                Vec::new()
            } else {
                let (data_fields, data_arrays, data_bitmap) =
                    (data.fields(), data.values(), data.validity());
                let is_valid = |i| data_bitmap.map_or(true, |bitmap| bitmap.get_bit(i));
                let arrays_by_name: ::std::collections::HashMap<_, _> = data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(data_arrays)
                    .collect();
                let origin = {
                    let data = &**arrays_by_name["origin"];

                    {
                        let data = data
                            .as_any()
                            .downcast_ref::<::arrow2::array::FixedSizeListArray>()
                            .unwrap();
                        if data . is_empty () { Vec :: new () }

 else { let bitmap = data . validity () . cloned () ; let offsets = (0 ..) . step_by (3usize) . zip ((3usize ..) . step_by (3usize) . take (data . len ())) ; let data = & * * data . values () ; let data = data . as_any () . downcast_ref :: < Float32Array > () . unwrap () . into_iter () . map (| v | v . copied ()) . map (| v | v . ok_or_else (|| crate :: DeserializationError :: MissingData { backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

)) . collect :: < crate :: DeserializationResult < Vec < _ >> > () ? ; offsets . enumerate () . map (move | (i , (start , end)) | bitmap . as_ref () . map_or (true , | bitmap | bitmap . get_bit (i)) . then (|| { data . get (start as usize .. end as usize) . ok_or (crate :: DeserializationError :: OffsetsMismatch { bounds : (start as usize , end as usize) , len : data . len () , backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

) ? . to_vec () . try_into () . map_err (| _err | crate :: DeserializationError :: ArrayLengthMismatch { expected : 3usize , got : (end - start) as usize , backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

) }

) . transpose ()) . map (| res | res . map (| opt | opt . map (| v | crate :: datatypes :: Vec3D (v)))) . collect :: < crate :: DeserializationResult < Vec < Option < _ >> >> () ? }

 . into_iter ()
                    }
                };
                let vector = {
                    let data = &**arrays_by_name["vector"];

                    {
                        let data = data
                            .as_any()
                            .downcast_ref::<::arrow2::array::FixedSizeListArray>()
                            .unwrap();
                        if data . is_empty () { Vec :: new () }

 else { let bitmap = data . validity () . cloned () ; let offsets = (0 ..) . step_by (3usize) . zip ((3usize ..) . step_by (3usize) . take (data . len ())) ; let data = & * * data . values () ; let data = data . as_any () . downcast_ref :: < Float32Array > () . unwrap () . into_iter () . map (| v | v . copied ()) . map (| v | v . ok_or_else (|| crate :: DeserializationError :: MissingData { backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

)) . collect :: < crate :: DeserializationResult < Vec < _ >> > () ? ; offsets . enumerate () . map (move | (i , (start , end)) | bitmap . as_ref () . map_or (true , | bitmap | bitmap . get_bit (i)) . then (|| { data . get (start as usize .. end as usize) . ok_or (crate :: DeserializationError :: OffsetsMismatch { bounds : (start as usize , end as usize) , len : data . len () , backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

) ? . to_vec () . try_into () . map_err (| _err | crate :: DeserializationError :: ArrayLengthMismatch { expected : 3usize , got : (end - start) as usize , backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

) }

) . transpose ()) . map (| res | res . map (| opt | opt . map (| v | crate :: datatypes :: Vec3D (v)))) . collect :: < crate :: DeserializationResult < Vec < Option < _ >> >> () ? }

 . into_iter ()
                    }
                };
                ::itertools::izip!(origin, vector)
                    .enumerate()
                    .map(|(i, (origin, vector))| {
                        is_valid(i)
                            .then(|| {
                                Ok(Self {
                                    origin: origin
                                        .ok_or_else(|| crate::DeserializationError::MissingData {
                                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                                        })
                                        .map_err(|err| crate::DeserializationError::Context {
                                            location: "rerun.datatypes.Arrow3D#origin".into(),
                                            source: Box::new(err),
                                        })?,
                                    vector: vector
                                        .ok_or_else(|| crate::DeserializationError::MissingData {
                                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                                        })
                                        .map_err(|err| crate::DeserializationError::Context {
                                            location: "rerun.datatypes.Arrow3D#vector".into(),
                                            source: Box::new(err),
                                        })?,
                                })
                            })
                            .transpose()
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.datatypes.Arrow3D".into(),
                        source: Box::new(err),
                    })?
            }
        })
    }

    #[inline]
    fn try_iter_from_arrow(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Self::Iter<'_>>
    where
        Self: Sized,
    {
        Ok(Box::new(Self::try_from_arrow_opt(data)?.into_iter()))
    }

    #[inline]
    fn convert_item_to_self(item: Self::Item<'_>) -> Option<Self> {
        item
    }
}

impl crate::Datatype for Arrow3D {}