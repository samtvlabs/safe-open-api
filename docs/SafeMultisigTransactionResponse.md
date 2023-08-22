# SafeMultisigTransactionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**safe** | **String** |  | 
**to** | **String** |  | 
**value** | **String** |  | 
**data** | Option<**String**> |  | [optional]
**operation** | **i32** |  | 
**gas_token** | Option<**String**> |  | [optional]
**safe_tx_gas** | **i32** |  | 
**base_gas** | **i32** |  | 
**gas_price** | **String** |  | 
**refund_receiver** | Option<**String**> |  | [optional]
**nonce** | **i32** |  | 
**execution_date** | **String** |  | 
**submission_date** | **String** |  | 
**modified** | **String** |  | 
**block_number** | Option<**i32**> |  | [optional][readonly]
**transaction_hash** | **String** |  | 
**safe_tx_hash** | **String** |  | 
**executor** | Option<**String**> |  | [optional][readonly]
**is_executed** | **bool** |  | 
**is_successful** | Option<**bool**> |  | [optional][readonly]
**eth_gas_price** | Option<**String**> |  | [optional][readonly]
**max_fee_per_gas** | Option<**String**> |  | [optional][readonly]
**max_priority_fee_per_gas** | Option<**String**> |  | [optional][readonly]
**gas_used** | Option<**i32**> |  | [optional][readonly]
**fee** | Option<**i32**> |  | [optional][readonly]
**origin** | Option<**String**> |  | [optional][readonly]
**data_decoded** | Option<**String**> |  | [optional][readonly]
**confirmations_required** | **i32** |  | 
**confirmations** | Option<[**crate::models::SafeMultisigConfirmationResponse**](SafeMultisigConfirmationResponse.md)> |  | [optional]
**trusted** | **bool** |  | 
**signatures** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


