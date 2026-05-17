output "resource_group_name" {
  description = "Created Azure resource group name."
  value       = module.resource_group.name
}

output "storage_account_name" {
  description = "Created Azure storage account name."
  value       = module.storage.storage_account_name
}

output "storage_container_name" {
  description = "Created Azure Blob container name."
  value       = module.storage.storage_container_name
}

output "state_container_name" {
  description = "Created Azure Blob container name for OpenTofu state."
  value       = module.storage.state_container_name
}

output "primary_blob_endpoint" {
  description = "Primary Blob service endpoint."
  value       = module.storage.primary_blob_endpoint
}

output "key_vault_name" {
  description = "Created Azure Key Vault name."
  value       = module.key_vault.name
}

output "key_vault_uri" {
  description = "Created Azure Key Vault URI."
  value       = module.key_vault.vault_uri
}
