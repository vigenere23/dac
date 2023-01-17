use serde::{Deserialize, Serialize};

use crate::params::{
    channel::ChannelParamsExtraItemsStrategy, permission::PermissionsOverwriteParams,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct CategoriesParamsList {
    #[serde(default = "Vec::default")]
    pub items: Vec<CategoryParams>,
    #[serde(default = "CategoryParamsExtraItemsStrategy::default")]
    pub extra_items: CategoryParamsExtraItemsStrategy,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "strategy")]
pub enum CategoryParamsExtraItemsStrategy {
    KEEP,
    REMOVE,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CategoryParams {
    pub name: String,
    #[serde(default = "Vec::default")]
    pub permissions_overwrites: Vec<PermissionsOverwriteParams>,
    #[serde(default = "bool::default")]
    pub sync_permissions: bool,
    #[serde(default = "ChannelParamsExtraItemsStrategy::default")]
    pub extra_channels: ChannelParamsExtraItemsStrategy,
}

impl Default for CategoryParamsExtraItemsStrategy {
    fn default() -> Self {
        Self::REMOVE
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        params::{
            category::{CategoriesParamsList, CategoryParams, CategoryParamsExtraItemsStrategy},
            channel::ChannelParamsExtraItemsStrategy,
            permission::PermissionsOverwriteParams,
        },
        permission::Permission,
    };

    #[test]
    fn it_parses_params_list() {
        let yaml_params_list = r"
            items:
            - name: category_1
              permissions_overwrites:
              - role: role_1
                allow: [ADMINISTRATOR]
                deny: [SEND_MESSAGES]
              sync_permissions: true
              extra_channels:
                strategy: KEEP
            extra_items:
              strategy: KEEP
        ";
        let expected_params_list = CategoriesParamsList {
            items: vec![CategoryParams {
                name: "category_1".to_string(),
                permissions_overwrites: vec![PermissionsOverwriteParams {
                    role: "role_1".to_string(),
                    allow: vec![Permission::ADMINISTRATOR],
                    deny: vec![Permission::SEND_MESSAGES],
                }],
                sync_permissions: true,
                extra_channels: ChannelParamsExtraItemsStrategy::KEEP,
            }],
            extra_items: CategoryParamsExtraItemsStrategy::KEEP,
        };

        let params_list: CategoriesParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, expected_params_list);
    }

    #[test]
    fn it_parses_empty_params_list_to_defaults() {
        let yaml_params_list = r"";

        let params_list: CategoriesParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, CategoriesParamsList::default());
    }

    #[test]
    fn it_parses_empty_params_fields_to_defaults() {
        let yaml_params_list = r"
            items:
            - name: category_1
        ";
        let expected_params_list = CategoriesParamsList {
            items: vec![CategoryParams {
                name: "category_1".to_string(),
                permissions_overwrites: vec![],
                sync_permissions: false,
                extra_channels: ChannelParamsExtraItemsStrategy::default(),
            }],
            extra_items: CategoryParamsExtraItemsStrategy::REMOVE,
        };

        let params_list: CategoriesParamsList = serde_yaml::from_str(yaml_params_list).unwrap();

        assert_eq!(params_list, expected_params_list);
    }
}