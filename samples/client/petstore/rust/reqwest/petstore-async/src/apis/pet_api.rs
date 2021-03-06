/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;
use std::borrow::Borrow;
use std::option::Option;

use reqwest;

use super::{Error, configuration};

/// struct for passing parameters to the method `add_pet`
#[derive(Clone, Debug)]
pub struct AddPetParams {
    /// Pet object that needs to be added to the store
    pub body: crate::models::Pet
}

/// struct for passing parameters to the method `delete_pet`
#[derive(Clone, Debug)]
pub struct DeletePetParams {
    /// Pet id to delete
    pub pet_id: i64,
    pub api_key: Option<String>
}

/// struct for passing parameters to the method `find_pets_by_status`
#[derive(Clone, Debug)]
pub struct FindPetsByStatusParams {
    /// Status values that need to be considered for filter
    pub status: Vec<String>
}

/// struct for passing parameters to the method `find_pets_by_tags`
#[derive(Clone, Debug)]
pub struct FindPetsByTagsParams {
    /// Tags to filter by
    pub tags: Vec<String>
}

/// struct for passing parameters to the method `get_pet_by_id`
#[derive(Clone, Debug)]
pub struct GetPetByIdParams {
    /// ID of pet to return
    pub pet_id: i64
}

/// struct for passing parameters to the method `update_pet`
#[derive(Clone, Debug)]
pub struct UpdatePetParams {
    /// Pet object that needs to be added to the store
    pub body: crate::models::Pet
}

/// struct for passing parameters to the method `update_pet_with_form`
#[derive(Clone, Debug)]
pub struct UpdatePetWithFormParams {
    /// ID of pet that needs to be updated
    pub pet_id: i64,
    /// Updated name of the pet
    pub name: Option<String>,
    /// Updated status of the pet
    pub status: Option<String>
}

/// struct for passing parameters to the method `upload_file`
#[derive(Clone, Debug)]
pub struct UploadFileParams {
    /// ID of pet to update
    pub pet_id: i64,
    /// Additional data to pass to server
    pub additional_metadata: Option<String>,
    /// file to upload
    pub file: Option<std::path::PathBuf>
}


    pub async fn add_pet(configuration: &configuration::Configuration, params: AddPetParams) -> Result<(), Error> {
        // unbox the parameters
        let body = params.body;

        let client = &configuration.client;

        let uri_str = format!("{}/pet", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn delete_pet(configuration: &configuration::Configuration, params: DeletePetParams) -> Result<(), Error> {
        // unbox the parameters
        let pet_id = params.pet_id;
        let api_key = params.api_key;

        let client = &configuration.client;

        let uri_str = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(param_value) = api_key {
            req_builder = req_builder.header("api_key", param_value.to_string());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn find_pets_by_status(configuration: &configuration::Configuration, params: FindPetsByStatusParams) -> Result<Vec<crate::models::Pet>, Error> {
        // unbox the parameters
        let status = params.status;

        let client = &configuration.client;

        let uri_str = format!("{}/pet/findByStatus", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("status", &status.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        Ok(client.execute(req).await?.error_for_status()?.json::<Vec<crate::models::Pet>>().await?)
    }

    pub async fn find_pets_by_tags(configuration: &configuration::Configuration, params: FindPetsByTagsParams) -> Result<Vec<crate::models::Pet>, Error> {
        // unbox the parameters
        let tags = params.tags;

        let client = &configuration.client;

        let uri_str = format!("{}/pet/findByTags", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("tags", &tags.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        Ok(client.execute(req).await?.error_for_status()?.json::<Vec<crate::models::Pet>>().await?)
    }

    pub async fn get_pet_by_id(configuration: &configuration::Configuration, params: GetPetByIdParams) -> Result<crate::models::Pet, Error> {
        // unbox the parameters
        let pet_id = params.pet_id;

        let client = &configuration.client;

        let uri_str = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("api_key", val);
        };

        let req = req_builder.build()?;
        Ok(client.execute(req).await?.error_for_status()?.json::<crate::models::Pet>().await?)
    }

    pub async fn update_pet(configuration: &configuration::Configuration, params: UpdatePetParams) -> Result<(), Error> {
        // unbox the parameters
        let body = params.body;

        let client = &configuration.client;

        let uri_str = format!("{}/pet", configuration.base_path);
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn update_pet_with_form(configuration: &configuration::Configuration, params: UpdatePetWithFormParams) -> Result<(), Error> {
        // unbox the parameters
        let pet_id = params.pet_id;
        let name = params.name;
        let status = params.status;

        let client = &configuration.client;

        let uri_str = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        if let Some(param_value) = name {
            form_params.insert("name", param_value.to_string());
        }
        if let Some(param_value) = status {
            form_params.insert("status", param_value.to_string());
        }
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn upload_file(configuration: &configuration::Configuration, params: UploadFileParams) -> Result<crate::models::ApiResponse, Error> {
        // unbox the parameters
        let pet_id = params.pet_id;
        let additional_metadata = params.additional_metadata;
        let file = params.file;

        let client = &configuration.client;

        let uri_str = format!("{}/pet/{petId}/uploadImage", configuration.base_path, petId=pet_id);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form = reqwest::multipart::Form::new();
        if let Some(param_value) = additional_metadata {
            form = form.text("additionalMetadata", param_value.to_string());
        }
        // TODO: support file upload for 'file' parameter
        req_builder = req_builder.multipart(form);

        let req = req_builder.build()?;
        Ok(client.execute(req).await?.error_for_status()?.json::<crate::models::ApiResponse>().await?)
    }

