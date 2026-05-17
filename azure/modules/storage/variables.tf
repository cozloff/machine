variable "resource_group_name" {
  description = "Name of the resource group that owns the storage account."
  type        = string
}

variable "location" {
  description = "Azure region for the storage account."
  type        = string
}

variable "storage_account_prefix" {
  description = "Lowercase prefix for the storage account name. Azure storage account names must be globally unique, 3-24 chars, lowercase letters and numbers only."
  type        = string

  validation {
    condition     = can(regex("^[a-z0-9]{3,16}$", var.storage_account_prefix))
    error_message = "storage_account_prefix must be 3-16 chars and contain only lowercase letters and numbers."
  }
}

variable "container_name" {
  description = "Name of the Blob container to create."
  type        = string
}

variable "state_container_name" {
  description = "Name of the Blob container used for OpenTofu state."
  type        = string
}

variable "tags" {
  description = "Tags applied to storage resources."
  type        = map(string)
  default     = {}
}
