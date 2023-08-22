# \V2Api

All URIs are relative to *https://safe-transaction-mainnet.safe.global/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_safes_collectibles_list**](V2Api.md#v2_safes_collectibles_list) | **GET** /v2/safes/{address}/collectibles/ | 



## v2_safes_collectibles_list

> Vec<crate::models::SafeCollectibleResponse> v2_safes_collectibles_list(address, limit, offset, trusted, exclude_spam)


Get collectibles (ERC721 tokens) and information about them

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**trusted** | Option<**bool**> | If `True` just trusted tokens will be returned |  |[default to false]
**exclude_spam** | Option<**bool**> | If `True` spam tokens will not be returned |  |[default to false]

### Return type

[**Vec<crate::models::SafeCollectibleResponse>**](SafeCollectibleResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

