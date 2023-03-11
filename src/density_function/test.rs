use crate::density_function::deserialize::DensityFunctionTree;

#[test]
fn parse_density_function() {
    // Some Deserializer.
    let d = &mut serde_json::Deserializer::from_str(SAMPLE);
    let result: Result<DensityFunctionTree, _> = serde_path_to_error::deserialize(d);
    assert!(result.is_ok())
}

const SAMPLE: &str = r#"
{
  "type": "minecraft:cache_once",
  "argument": {
    "type": "minecraft:min",
    "argument1": {
      "type": "minecraft:add",
      "argument1": {
        "type": "minecraft:add",
        "argument1": 0.37,
        "argument2": {
          "type": "minecraft:noise",
          "noise": "minecraft:cave_entrance",
          "xz_scale": 0.75,
          "y_scale": 0.5
        }
      },
      "argument2": {
        "type": "minecraft:y_clamped_gradient",
        "from_y": -10,
        "to_y": 30,
        "from_value": 0.3,
        "to_value": 0
      }
    },
    "argument2": {
      "type": "minecraft:add",
      "argument1": "minecraft:overworld/caves/spaghetti_roughness_function",
      "argument2": {
        "type": "minecraft:clamp",
        "input": {
          "type": "minecraft:add",
          "argument1": {
            "type": "minecraft:max",
            "argument1": {
              "type": "minecraft:weird_scaled_sampler",
              "rarity_value_mapper": "type_1",
              "noise": "minecraft:spaghetti_3d_1",
              "input": {
                "type": "minecraft:cache_once",
                "argument": {
                  "type": "minecraft:noise",
                  "noise": "minecraft:spaghetti_3d_rarity",
                  "xz_scale": 2,
                  "y_scale": 1
                }
              }
            },
            "argument2": {
              "type": "minecraft:weird_scaled_sampler",
              "rarity_value_mapper": "type_1",
              "noise": "minecraft:spaghetti_3d_2",
              "input": {
                "type": "minecraft:cache_once",
                "argument": {
                  "type": "minecraft:noise",
                  "noise": "minecraft:spaghetti_3d_rarity",
                  "xz_scale": 2,
                  "y_scale": 1
                }
              }
            }
          },
          "argument2": {
            "type": "minecraft:add",
            "argument1": -0.0765,
            "argument2": {
              "type": "minecraft:mul",
              "argument1": -0.011499999999999996,
              "argument2": {
                "type": "minecraft:noise",
                "noise": "minecraft:spaghetti_3d_thickness",
                "xz_scale": 1,
                "y_scale": 1
              }
            }
          }
        },
        "min": -1,
        "max": 1
      }
    }
  }
}
"#;