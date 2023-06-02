# Rust API client for openapi

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**delete_client_app_get_delete**](docs/DefaultApi.md#delete_client_app_get_delete) | **DELETE** /client_apps/{id} | 
*DefaultApi* | [**get_client_app_get**](docs/DefaultApi.md#get_client_app_get) | **GET** /client_apps/{id} | 
*DefaultApi* | [**get_client_app_get_all**](docs/DefaultApi.md#get_client_app_get_all) | **GET** /client_apps | 
*DefaultApi* | [**get_docs**](docs/DefaultApi.md#get_docs) | **GET** /docs | 
*DefaultApi* | [**get_health_check**](docs/DefaultApi.md#get_health_check) | **GET** /health_check | 
*DefaultApi* | [**get_swagger_json**](docs/DefaultApi.md#get_swagger_json) | **GET** /docs/swagger.json | 
*DefaultApi* | [**post_client_app_post**](docs/DefaultApi.md#post_client_app_post) | **POST** /client_apps/{id} | 
*DefaultApi* | [**put_client_app_put**](docs/DefaultApi.md#put_client_app_put) | **PUT** /client_apps | 


## Documentation For Models

 - [ClientAppSchema](docs/ClientAppSchema.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



