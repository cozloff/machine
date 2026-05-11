output "resource_group_name" {
  description = "Created Azure resource group name."
  value       = azurerm_resource_group.this.name
}

output "storage_account_name" {
  description = "Created Azure storage account name."
  value       = azurerm_storage_account.this.name
}

output "storage_container_name" {
  description = "Created Azure Blob container name."
  value       = azurerm_storage_container.this.name
}

output "state_container_name" {
  description = "Created Azure Blob container name for OpenTofu state."
  value       = azurerm_storage_container.state.name
}

output "primary_blob_endpoint" {
  description = "Primary Blob service endpoint."
  value       = azurerm_storage_account.this.primary_blob_endpoint
}
