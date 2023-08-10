# Rust API client for labs_alert_manager_client

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `labs_alert_manager_client` and add the following to `Cargo.toml` under `[dependencies]`:

```
labs_alert_manager_client = { path = "./labs_alert_manager_client" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ChannelsApi* | [**delete_policy_channels_delete**](docs/ChannelsApi.md#delete_policy_channels_delete) | **DELETE** /channels/{id} | 
*ChannelsApi* | [**get_policy_channels_get**](docs/ChannelsApi.md#get_policy_channels_get) | **GET** /channels/{id} | 
*ChannelsApi* | [**get_policy_channels_get_all**](docs/ChannelsApi.md#get_policy_channels_get_all) | **GET** /channels | 
*ChannelsApi* | [**post_policy_channels_post**](docs/ChannelsApi.md#post_policy_channels_post) | **POST** /channels | 
*ChannelsApi* | [**post_policy_channels_test**](docs/ChannelsApi.md#post_policy_channels_test) | **POST** /channels/test | 
*ChannelsApi* | [**put_policy_channels_put**](docs/ChannelsApi.md#put_policy_channels_put) | **PUT** /channels/{id} | 
*DocsApi* | [**get_swagger_json**](docs/DocsApi.md#get_swagger_json) | **GET** /docs/swagger.json | 
*DocsApi* | [**get_swagger_yml**](docs/DocsApi.md#get_swagger_yml) | **GET** /docs/swagger.yml | 
*EventsApi* | [**get_events_get**](docs/EventsApi.md#get_events_get) | **GET** /events/{id} | 
*EventsApi* | [**get_events_get_all**](docs/EventsApi.md#get_events_get_all) | **GET** /events | 
*EventsApi* | [**get_events_parameters_get**](docs/EventsApi.md#get_events_parameters_get) | **GET** /events/parameters | 
*EventsApi* | [**post_events_post**](docs/EventsApi.md#post_events_post) | **POST** /events | 
*HealthCheckApi* | [**get_health_check**](docs/HealthCheckApi.md#get_health_check) | **GET** /health_check | 
*NotificationsApi* | [**get_notification_log_get**](docs/NotificationsApi.md#get_notification_log_get) | **GET** /notifications/{id} | 
*NotificationsApi* | [**get_notification_log_get_all**](docs/NotificationsApi.md#get_notification_log_get_all) | **GET** /notifications | 
*PoliciesApi* | [**delete_policies_delete**](docs/PoliciesApi.md#delete_policies_delete) | **DELETE** /policies/{id} | 
*PoliciesApi* | [**get_policies_get**](docs/PoliciesApi.md#get_policies_get) | **GET** /policies/{id} | 
*PoliciesApi* | [**get_policies_get_all**](docs/PoliciesApi.md#get_policies_get_all) | **GET** /policies | 
*PoliciesApi* | [**post_policies_post**](docs/PoliciesApi.md#post_policies_post) | **POST** /policies | 
*PoliciesApi* | [**put_policies_put**](docs/PoliciesApi.md#put_policies_put) | **PUT** /policies/{id} | 
*WelcomeApi* | [**get_welcome**](docs/WelcomeApi.md#get_welcome) | **GET** / | 


## Documentation For Models

 - [EventSchema](docs/EventSchema.md)
 - [PaginationSchema](docs/PaginationSchema.md)
 - [PolicyChannelSchema](docs/PolicyChannelSchema.md)
 - [PolicySchema](docs/PolicySchema.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



