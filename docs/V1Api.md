# \V1Api

All URIs are relative to *https://safe-transaction-mainnet.safe.global/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_about_ethereum_rpc_list**](V1Api.md#v1_about_ethereum_rpc_list) | **GET** /v1/about/ethereum-rpc/ | 
[**v1_about_ethereum_tracing_rpc_list**](V1Api.md#v1_about_ethereum_tracing_rpc_list) | **GET** /v1/about/ethereum-tracing-rpc/ | 
[**v1_about_indexing_list**](V1Api.md#v1_about_indexing_list) | **GET** /v1/about/indexing/ | 
[**v1_about_list**](V1Api.md#v1_about_list) | **GET** /v1/about/ | 
[**v1_about_master_copies_list**](V1Api.md#v1_about_master_copies_list) | **GET** /v1/about/master-copies/ | 
[**v1_contracts_list**](V1Api.md#v1_contracts_list) | **GET** /v1/contracts/ | 
[**v1_contracts_read**](V1Api.md#v1_contracts_read) | **GET** /v1/contracts/{address}/ | 
[**v1_data_decoder_create**](V1Api.md#v1_data_decoder_create) | **POST** /v1/data-decoder/ | 
[**v1_delegates_create**](V1Api.md#v1_delegates_create) | **POST** /v1/delegates/ | 
[**v1_delegates_delete**](V1Api.md#v1_delegates_delete) | **DELETE** /v1/delegates/{delegate_address}/ | 
[**v1_delegates_list**](V1Api.md#v1_delegates_list) | **GET** /v1/delegates/ | 
[**v1_messages_read**](V1Api.md#v1_messages_read) | **GET** /v1/messages/{message_hash}/ | 
[**v1_messages_signatures_create**](V1Api.md#v1_messages_signatures_create) | **POST** /v1/messages/{message_hash}/signatures/ | 
[**v1_module_transaction_read**](V1Api.md#v1_module_transaction_read) | **GET** /v1/module-transaction/{module_transaction_id} | 
[**v1_modules_safes_list**](V1Api.md#v1_modules_safes_list) | **GET** /v1/modules/{address}/safes/ | 
[**v1_multisig_transactions_confirmations_create**](V1Api.md#v1_multisig_transactions_confirmations_create) | **POST** /v1/multisig-transactions/{safe_tx_hash}/confirmations/ | 
[**v1_multisig_transactions_confirmations_list**](V1Api.md#v1_multisig_transactions_confirmations_list) | **GET** /v1/multisig-transactions/{safe_tx_hash}/confirmations/ | 
[**v1_multisig_transactions_read**](V1Api.md#v1_multisig_transactions_read) | **GET** /v1/multisig-transactions/{safe_tx_hash}/ | 
[**v1_notifications_devices_create**](V1Api.md#v1_notifications_devices_create) | **POST** /v1/notifications/devices/ | 
[**v1_notifications_devices_delete**](V1Api.md#v1_notifications_devices_delete) | **DELETE** /v1/notifications/devices/{uuid}/ | 
[**v1_notifications_devices_safes_delete**](V1Api.md#v1_notifications_devices_safes_delete) | **DELETE** /v1/notifications/devices/{uuid}/safes/{address}/ | 
[**v1_owners_safes_list**](V1Api.md#v1_owners_safes_list) | **GET** /v1/owners/{address}/safes/ | 
[**v1_safes_all_transactions_list**](V1Api.md#v1_safes_all_transactions_list) | **GET** /v1/safes/{address}/all-transactions/ | 
[**v1_safes_balances_list**](V1Api.md#v1_safes_balances_list) | **GET** /v1/safes/{address}/balances/ | 
[**v1_safes_balances_usd_list**](V1Api.md#v1_safes_balances_usd_list) | **GET** /v1/safes/{address}/balances/usd/ | 
[**v1_safes_creation_list**](V1Api.md#v1_safes_creation_list) | **GET** /v1/safes/{address}/creation/ | 
[**v1_safes_delegates_delete**](V1Api.md#v1_safes_delegates_delete) | **DELETE** /v1/safes/{address}/delegates/{delegate_address}/ | 
[**v1_safes_incoming_transfers_list**](V1Api.md#v1_safes_incoming_transfers_list) | **GET** /v1/safes/{address}/incoming-transfers/ | 
[**v1_safes_messages_create**](V1Api.md#v1_safes_messages_create) | **POST** /v1/safes/{address}/messages/ | 
[**v1_safes_messages_list**](V1Api.md#v1_safes_messages_list) | **GET** /v1/safes/{address}/messages/ | 
[**v1_safes_module_transactions_list**](V1Api.md#v1_safes_module_transactions_list) | **GET** /v1/safes/{address}/module-transactions/ | 
[**v1_safes_multisig_transactions_create**](V1Api.md#v1_safes_multisig_transactions_create) | **POST** /v1/safes/{address}/multisig-transactions/ | 
[**v1_safes_multisig_transactions_estimations_create**](V1Api.md#v1_safes_multisig_transactions_estimations_create) | **POST** /v1/safes/{address}/multisig-transactions/estimations/ | 
[**v1_safes_multisig_transactions_list**](V1Api.md#v1_safes_multisig_transactions_list) | **GET** /v1/safes/{address}/multisig-transactions/ | 
[**v1_safes_read**](V1Api.md#v1_safes_read) | **GET** /v1/safes/{address}/ | 
[**v1_safes_transactions_create**](V1Api.md#v1_safes_transactions_create) | **POST** /v1/safes/{address}/transactions/ | 
[**v1_safes_transactions_list**](V1Api.md#v1_safes_transactions_list) | **GET** /v1/safes/{address}/transactions/ | 
[**v1_safes_transfers_list**](V1Api.md#v1_safes_transfers_list) | **GET** /v1/safes/{address}/transfers/ | 
[**v1_tokens_list**](V1Api.md#v1_tokens_list) | **GET** /v1/tokens/ | 
[**v1_tokens_prices_usd_read**](V1Api.md#v1_tokens_prices_usd_read) | **GET** /v1/tokens/{address}/prices/usd/ | 
[**v1_tokens_read**](V1Api.md#v1_tokens_read) | **GET** /v1/tokens/{address}/ | 
[**v1_transactions_read**](V1Api.md#v1_transactions_read) | **GET** /v1/transactions/{safe_tx_hash}/ | 
[**v1_transfer_read**](V1Api.md#v1_transfer_read) | **GET** /v1/transfer/{transfer_id} | 



## v1_about_ethereum_rpc_list

> v1_about_ethereum_rpc_list()


Get information about the Ethereum RPC node used by the service

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_about_ethereum_tracing_rpc_list

> v1_about_ethereum_tracing_rpc_list()


Get information about the Ethereum Tracing RPC node used by the service (if any configured)

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_about_indexing_list

> Vec<crate::models::IndexingStatus> v1_about_indexing_list()


Get current indexing status for ERC20/721 events

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::IndexingStatus>**](IndexingStatus.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_about_list

> v1_about_list()


Returns information and configuration of the service

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_about_master_copies_list

> Vec<crate::models::MasterCopyResponse> v1_about_master_copies_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::MasterCopyResponse>**](MasterCopyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_contracts_list

> crate::models::V1ContractsList200Response v1_contracts_list(ordering, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::V1ContractsList200Response**](v1_contracts_list_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_contracts_read

> crate::models::Contract v1_contracts_read(address)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | A unique value identifying this contract. | [required] |

### Return type

[**crate::models::Contract**](Contract.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_data_decoder_create

> v1_data_decoder_create(data)


Returns decoded information using tx service internal ABI information given the tx data as a `0x` prefixed hexadecimal string. If address of the receiving contract is provided decoded data will be more accurate, as in case of ABI collision service will know which ABI to use.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**DataDecoder**](DataDecoder.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_delegates_create

> v1_delegates_create(data)


Create a delegate for a Safe address with a custom label. Calls with same delegate but different label or signer will update the label or delegator if different. For the signature we are using TOTP with `T0=0` and `Tx=3600`. TOTP is calculated by taking the Unix UTC epoch time (no milliseconds) and dividing by 3600 (natural division, no decimals) For signature this hash need to be signed: keccak(checksummed address + str(int(current_epoch // 3600))) For example:      - We want to add the delegate `0x132512f995866CcE1b0092384A6118EDaF4508Ff` and `epoch=1586779140`.      - `TOTP = epoch // 3600 = 1586779140 // 3600 = 440771`      - The hash to sign by a Safe owner would be `keccak(\"0x132512f995866CcE1b0092384A6118EDaF4508Ff440771\")`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**Delegate**](Delegate.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_delegates_delete

> v1_delegates_delete(delegate_address, data)


Delete every pair delegate/delegator found. Signature is built the same way as for adding a delegate, but in this case the signer can be either the `delegator` (owner) or the `delegate` itself. Check `POST /delegates/`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delegate_address** | **String** |  | [required] |
**data** | [**DelegateDelete**](DelegateDelete.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_delegates_list

> crate::models::V1DelegatesList200Response v1_delegates_list(safe, delegate, delegator, label, limit, offset)


Get list of delegates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**safe** | Option<**String**> | safe |  |
**delegate** | Option<**String**> | delegate |  |
**delegator** | Option<**String**> | delegator |  |
**label** | Option<**String**> | label |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::V1DelegatesList200Response**](v1_delegates_list_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_messages_read

> crate::models::SafeMessageResponse v1_messages_read(message_hash)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_hash** | **std::path::PathBuf** | A unique value identifying this safe message. | [required] |

### Return type

[**crate::models::SafeMessageResponse**](SafeMessageResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_messages_signatures_create

> v1_messages_signatures_create(message_hash, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_hash** | **String** |  | [required] |
**data** | [**SafeMessageSignature**](SafeMessageSignature.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_module_transaction_read

> crate::models::SafeModuleTransactionResponse v1_module_transaction_read(module_transaction_id)


:return: module transaction filtered by module_transaction_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_transaction_id** | **String** |  | [required] |

### Return type

[**crate::models::SafeModuleTransactionResponse**](SafeModuleTransactionResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_modules_safes_list

> crate::models::ModulesResponse v1_modules_safes_list(address)


Return Safes where the module address provided is enabled

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

[**crate::models::ModulesResponse**](ModulesResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_multisig_transactions_confirmations_create

> v1_multisig_transactions_confirmations_create(safe_tx_hash, data)


Add a confirmation for a transaction. More than one signature can be used. This endpoint does not support the use of delegates to make a transaction trusted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**safe_tx_hash** | **String** |  | [required] |
**data** | [**SafeMultisigConfirmation**](SafeMultisigConfirmation.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_multisig_transactions_confirmations_list

> crate::models::V1MultisigTransactionsConfirmationsList200Response v1_multisig_transactions_confirmations_list(safe_tx_hash, limit, offset)


Get the list of confirmations for a multisig transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**safe_tx_hash** | **String** |  | [required] |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::V1MultisigTransactionsConfirmationsList200Response**](v1_multisig_transactions_confirmations_list_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_multisig_transactions_read

> crate::models::SafeMultisigTransactionResponse v1_multisig_transactions_read(safe_tx_hash)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**safe_tx_hash** | **String** |  | [required] |

### Return type

[**crate::models::SafeMultisigTransactionResponse**](SafeMultisigTransactionResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_notifications_devices_create

> crate::models::FirebaseDeviceSerializerWithOwnersResponse v1_notifications_devices_create(data)


Creates a new FirebaseDevice. If uuid is not provided a new device will be created. If a uuid for an existing Safe is provided the FirebaseDevice will be updated with all the new data provided. Safes provided on the request are always added and never removed/replaced Signature must sign `keccack256('gnosis-safe{timestamp-epoch}{uuid}{cloud_messaging_token}{safes_sorted}':     - `{timestamp-epoch}` must be an integer (no milliseconds)     - `{safes_sorted}` must be checksummed safe addresses sorted and joined with no spaces

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**FirebaseDevice**](FirebaseDevice.md) |  | [required] |

### Return type

[**crate::models::FirebaseDeviceSerializerWithOwnersResponse**](FirebaseDeviceSerializerWithOwnersResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_notifications_devices_delete

> v1_notifications_devices_delete(uuid)


Remove a FirebaseDevice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Firebase Device. | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_notifications_devices_safes_delete

> v1_notifications_devices_safes_delete(uuid, address)


Remove a Safe for a FirebaseDevice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Firebase Device. | [required] |
**address** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_owners_safes_list

> crate::models::OwnerResponse v1_owners_safes_list(address)


Return Safes where the address provided is an owner

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

[**crate::models::OwnerResponse**](OwnerResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_all_transactions_list

> crate::models::AllTransactionsSchema v1_safes_all_transactions_list(address, ordering, limit, offset, executed, queued, trusted)


Returns a paginated list of transactions for a Safe. The list has different structures depending on the transaction type: - Multisig Transactions for a Safe. `tx_type=MULTISIG_TRANSACTION`. If the query parameter `queued=False` is set only the transactions with `safe nonce < current Safe nonce` will be displayed. By default, only the `trusted` transactions will be displayed (transactions indexed, with at least one confirmation or proposed by a delegate). If you need that behaviour to be disabled set the query parameter `trusted=False` - Module Transactions for a Safe. `tx_type=MODULE_TRANSACTION` - Incoming Transfers of Ether/ERC20 Tokens/ERC721 Tokens. `tx_type=ETHEREUM_TRANSACTION`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**executed** | Option<**bool**> | If `True` only executed transactions are returned |  |[default to false]
**queued** | Option<**bool**> | If `True` transactions with `nonce >= Safe current nonce` are also returned |  |[default to true]
**trusted** | Option<**bool**> | If `True` just trusted transactions are shown (indexed, added by a delegate or with at least one confirmation) |  |[default to true]

### Return type

[**crate::models::AllTransactionsSchema**](AllTransactionsSchema.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_balances_list

> Vec<crate::models::SafeBalanceResponse> v1_safes_balances_list(address, trusted, exclude_spam)


Get balance for Ether and ERC20 tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**trusted** | Option<**bool**> | If `True` just trusted tokens will be returned |  |[default to false]
**exclude_spam** | Option<**bool**> | If `True` spam tokens will not be returned |  |[default to false]

### Return type

[**Vec<crate::models::SafeBalanceResponse>**](SafeBalanceResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_balances_usd_list

> Vec<crate::models::SafeBalanceUsdResponse> v1_safes_balances_usd_list(address, trusted, exclude_spam)


Get balance for Ether and ERC20 tokens with USD fiat conversion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**trusted** | Option<**bool**> | If `True` just trusted tokens will be returned |  |[default to false]
**exclude_spam** | Option<**bool**> | If `True` spam tokens will not be returned |  |[default to false]

### Return type

[**Vec<crate::models::SafeBalanceUsdResponse>**](SafeBalanceUsdResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_creation_list

> crate::models::SafeCreationInfoResponse v1_safes_creation_list(address)


Get status of the safe

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

[**crate::models::SafeCreationInfoResponse**](SafeCreationInfoResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_delegates_delete

> v1_safes_delegates_delete(address, delegate_address, data)


Delete a delegate for a Safe. Signature is built the same way that for adding a delegate. Check `POST /delegates/`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**delegate_address** | **String** |  | [required] |
**data** | [**SafeDelegateDelete**](SafeDelegateDelete.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_incoming_transfers_list

> Vec<crate::models::TransferWithTokenInfoResponse> v1_safes_incoming_transfers_list(address, _from, block_number, block_number__gt, block_number__lt, execution_date__gte, execution_date__lte, execution_date__gt, execution_date__lt, to, token_address, transaction_hash, value, value__gt, value__lt, erc20, erc721, ether, limit, offset)


Returns incoming ether/tokens transfers for a Safe

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**_from** | Option<**String**> | _from |  |
**block_number** | Option<**String**> | block_number |  |
**block_number__gt** | Option<**String**> | block_number__gt |  |
**block_number__lt** | Option<**String**> | block_number__lt |  |
**execution_date__gte** | Option<**String**> | execution_date__gte |  |
**execution_date__lte** | Option<**String**> | execution_date__lte |  |
**execution_date__gt** | Option<**String**> | execution_date__gt |  |
**execution_date__lt** | Option<**String**> | execution_date__lt |  |
**to** | Option<**String**> | to |  |
**token_address** | Option<**String**> | token_address |  |
**transaction_hash** | Option<**String**> | transaction_hash |  |
**value** | Option<**String**> | value |  |
**value__gt** | Option<**String**> | value__gt |  |
**value__lt** | Option<**String**> | value__lt |  |
**erc20** | Option<**String**> | erc20 |  |
**erc721** | Option<**String**> | erc721 |  |
**ether** | Option<**String**> | ether |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**Vec<crate::models::TransferWithTokenInfoResponse>**](TransferWithTokenInfoResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_messages_create

> v1_safes_messages_create(address, data)


Create a new signed message for a Safe. Message can be: - A ``string``, so ``EIP191`` will be used to get the hash. - An ``EIP712`` ``object``.  Hash will be calculated from the provided ``message``. Sending a raw ``hash`` will not be accepted, service needs to derive it itself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**data** | [**SafeMessage**](SafeMessage.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_messages_list

> crate::models::V1SafesMessagesList200Response v1_safes_messages_list(address, ordering, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::V1SafesMessagesList200Response**](v1_safes_messages_list_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_module_transactions_list

> crate::models::V1SafesModuleTransactionsList200Response v1_safes_module_transactions_list(address, safe, module, to, operation, failed, block_number, block_number__gt, block_number__lt, transaction_hash, ordering, limit, offset)


Returns the module transaction of a Safe

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**safe** | Option<**String**> | safe |  |
**module** | Option<**String**> | module |  |
**to** | Option<**String**> | to |  |
**operation** | Option<**String**> | operation |  |
**failed** | Option<**String**> | failed |  |
**block_number** | Option<**String**> | block_number |  |
**block_number__gt** | Option<**String**> | block_number__gt |  |
**block_number__lt** | Option<**String**> | block_number__lt |  |
**transaction_hash** | Option<**String**> | transaction_hash |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::V1SafesModuleTransactionsList200Response**](v1_safes_module_transactions_list_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_multisig_transactions_create

> v1_safes_multisig_transactions_create(address, data)


Creates a Multisig Transaction with its confirmations and retrieves all the information related.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**data** | [**SafeMultisigTransaction**](SafeMultisigTransaction.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_multisig_transactions_estimations_create

> crate::models::SafeMultisigTransactionEstimateResponse v1_safes_multisig_transactions_estimations_create(address, data)


Estimates `safeTxGas` for a Safe Multisig Transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**data** | [**SafeMultisigTransactionEstimate**](SafeMultisigTransactionEstimate.md) |  | [required] |

### Return type

[**crate::models::SafeMultisigTransactionEstimateResponse**](SafeMultisigTransactionEstimateResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_multisig_transactions_list

> crate::models::V1SafesMultisigTransactionsList200Response v1_safes_multisig_transactions_list(address, failed, modified__lt, modified__gt, modified__lte, modified__gte, nonce__lt, nonce__gt, nonce__lte, nonce__gte, nonce, safe_tx_hash, to, value__lt, value__gt, value, executed, has_confirmations, trusted, execution_date__gte, execution_date__lte, submission_date__gte, submission_date__lte, transaction_hash, ordering, limit, offset)


Returns the history of a multisig tx (safe)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**failed** | Option<**String**> | failed |  |
**modified__lt** | Option<**String**> | modified__lt |  |
**modified__gt** | Option<**String**> | modified__gt |  |
**modified__lte** | Option<**String**> | modified__lte |  |
**modified__gte** | Option<**String**> | modified__gte |  |
**nonce__lt** | Option<**String**> | nonce__lt |  |
**nonce__gt** | Option<**String**> | nonce__gt |  |
**nonce__lte** | Option<**String**> | nonce__lte |  |
**nonce__gte** | Option<**String**> | nonce__gte |  |
**nonce** | Option<**String**> | nonce |  |
**safe_tx_hash** | Option<**String**> | safe_tx_hash |  |
**to** | Option<**String**> | to |  |
**value__lt** | Option<**String**> | value__lt |  |
**value__gt** | Option<**String**> | value__gt |  |
**value** | Option<**String**> | value |  |
**executed** | Option<**String**> | executed |  |
**has_confirmations** | Option<**String**> | has_confirmations |  |
**trusted** | Option<**String**> | trusted |  |
**execution_date__gte** | Option<**String**> | execution_date__gte |  |
**execution_date__lte** | Option<**String**> | execution_date__lte |  |
**submission_date__gte** | Option<**String**> | submission_date__gte |  |
**submission_date__lte** | Option<**String**> | submission_date__lte |  |
**transaction_hash** | Option<**String**> | transaction_hash |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::V1SafesMultisigTransactionsList200Response**](v1_safes_multisig_transactions_list_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_read

> crate::models::SafeInfoResponse v1_safes_read(address)


Get status of the safe

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

[**crate::models::SafeInfoResponse**](SafeInfoResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_transactions_create

> crate::models::SafeMultisigTransaction v1_safes_transactions_create(address, data)


Use `multisig-transactions` instead of `transactions`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**data** | [**SafeMultisigTransaction**](SafeMultisigTransaction.md) |  | [required] |

### Return type

[**crate::models::SafeMultisigTransaction**](SafeMultisigTransaction.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_transactions_list

> crate::models::V1SafesMultisigTransactionsList200Response v1_safes_transactions_list(address, failed, modified__lt, modified__gt, modified__lte, modified__gte, nonce__lt, nonce__gt, nonce__lte, nonce__gte, nonce, safe_tx_hash, to, value__lt, value__gt, value, executed, has_confirmations, trusted, execution_date__gte, execution_date__lte, submission_date__gte, submission_date__lte, transaction_hash, ordering, limit, offset)


Use `multisig-transactions` instead of `transactions`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**failed** | Option<**String**> | failed |  |
**modified__lt** | Option<**String**> | modified__lt |  |
**modified__gt** | Option<**String**> | modified__gt |  |
**modified__lte** | Option<**String**> | modified__lte |  |
**modified__gte** | Option<**String**> | modified__gte |  |
**nonce__lt** | Option<**String**> | nonce__lt |  |
**nonce__gt** | Option<**String**> | nonce__gt |  |
**nonce__lte** | Option<**String**> | nonce__lte |  |
**nonce__gte** | Option<**String**> | nonce__gte |  |
**nonce** | Option<**String**> | nonce |  |
**safe_tx_hash** | Option<**String**> | safe_tx_hash |  |
**to** | Option<**String**> | to |  |
**value__lt** | Option<**String**> | value__lt |  |
**value__gt** | Option<**String**> | value__gt |  |
**value** | Option<**String**> | value |  |
**executed** | Option<**String**> | executed |  |
**has_confirmations** | Option<**String**> | has_confirmations |  |
**trusted** | Option<**String**> | trusted |  |
**execution_date__gte** | Option<**String**> | execution_date__gte |  |
**execution_date__lte** | Option<**String**> | execution_date__lte |  |
**submission_date__gte** | Option<**String**> | submission_date__gte |  |
**submission_date__lte** | Option<**String**> | submission_date__lte |  |
**transaction_hash** | Option<**String**> | transaction_hash |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::V1SafesMultisigTransactionsList200Response**](v1_safes_multisig_transactions_list_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_safes_transfers_list

> Vec<crate::models::TransferWithTokenInfoResponse> v1_safes_transfers_list(address, _from, block_number, block_number__gt, block_number__lt, execution_date__gte, execution_date__lte, execution_date__gt, execution_date__lt, to, token_address, transaction_hash, value, value__gt, value__lt, erc20, erc721, ether, limit, offset)


Returns ether/tokens transfers for a Safe

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |
**_from** | Option<**String**> | _from |  |
**block_number** | Option<**String**> | block_number |  |
**block_number__gt** | Option<**String**> | block_number__gt |  |
**block_number__lt** | Option<**String**> | block_number__lt |  |
**execution_date__gte** | Option<**String**> | execution_date__gte |  |
**execution_date__lte** | Option<**String**> | execution_date__lte |  |
**execution_date__gt** | Option<**String**> | execution_date__gt |  |
**execution_date__lt** | Option<**String**> | execution_date__lt |  |
**to** | Option<**String**> | to |  |
**token_address** | Option<**String**> | token_address |  |
**transaction_hash** | Option<**String**> | transaction_hash |  |
**value** | Option<**String**> | value |  |
**value__gt** | Option<**String**> | value__gt |  |
**value__lt** | Option<**String**> | value__lt |  |
**erc20** | Option<**String**> | erc20 |  |
**erc721** | Option<**String**> | erc721 |  |
**ether** | Option<**String**> | ether |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**Vec<crate::models::TransferWithTokenInfoResponse>**](TransferWithTokenInfoResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_tokens_list

> crate::models::V1TokensList200Response v1_tokens_list(name, address, symbol, decimals__lt, decimals__gt, decimals, search, ordering, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | name |  |
**address** | Option<**String**> | address |  |
**symbol** | Option<**String**> | symbol |  |
**decimals__lt** | Option<**String**> | decimals__lt |  |
**decimals__gt** | Option<**String**> | decimals__gt |  |
**decimals** | Option<**String**> | decimals |  |
**search** | Option<**String**> | A search term. |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::V1TokensList200Response**](v1_tokens_list_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_tokens_prices_usd_read

> crate::models::TokenPriceResponse v1_tokens_prices_usd_read(address)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | A unique value identifying this token. | [required] |

### Return type

[**crate::models::TokenPriceResponse**](TokenPriceResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_tokens_read

> crate::models::TokenInfoResponse v1_tokens_read(address)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | A unique value identifying this token. | [required] |

### Return type

[**crate::models::TokenInfoResponse**](TokenInfoResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_transactions_read

> v1_transactions_read(safe_tx_hash)


Use `multisig-transactions` instead of `transactions`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**safe_tx_hash** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_transfer_read

> crate::models::TransferWithTokenInfoResponse v1_transfer_read(transfer_id)


:return: transfer filtered by transfer_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_id** | **String** |  | [required] |

### Return type

[**crate::models::TransferWithTokenInfoResponse**](TransferWithTokenInfoResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

