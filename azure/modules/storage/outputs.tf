output "storage_account_name" {
  description = "Storage account name."
  value       = azurerm_storage_account.this.name
}

output "storage_account_id" {
  description = "Storage account resource ID."
  value       = azurerm_storage_account.this.id
}

output "storage_container_name" {
  description = "Primary Blob container name."
  value       = azurerm_storage_container.this.name
}

output "state_container_name" {
  description = "OpenTofu state Blob container name."
  value       = azurerm_storage_container.state.name
}

output "primary_blob_endpoint" {
  description = "Primary Blob service endpoint."
  value       = azurerm_storage_account.this.primary_blob_endpoint
}
