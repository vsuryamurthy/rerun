// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs"

#pragma once

#include <cstdint>
#include <memory>
#include <rerun/data_cell.hpp>
#include <rerun/result.hpp>
#include <string>
#include <utility>
#include <vector>

namespace arrow {
    class DataType;
    class ListBuilder;
    class MemoryPool;
} // namespace arrow

namespace rerun {
    namespace components {
        struct AffixFuzzer12 {
            std::vector<std::string> many_strings_required;

            /// Name of the component, used for serialization.
            static const char* NAME;

          public:
            AffixFuzzer12() = default;

            AffixFuzzer12(std::vector<std::string> _many_strings_required)
                : many_strings_required(std::move(_many_strings_required)) {}

            AffixFuzzer12& operator=(std::vector<std::string> _many_strings_required) {
                many_strings_required = std::move(_many_strings_required);
                return *this;
            }

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& to_arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static Result<std::shared_ptr<arrow::ListBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static Error fill_arrow_array_builder(
                arrow::ListBuilder* builder, const AffixFuzzer12* elements, size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of AffixFuzzer12 components.
            static Result<rerun::DataCell> to_data_cell(
                const AffixFuzzer12* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rerun