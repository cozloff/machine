variable "resource_group_name" {
  description = "Name of the Azure resource group to create."
  type        = string
  default     = "rg-machine-dev"
}

variable "location" {
  description = "Azure region for the resource group and storage account."
  type        = string
  default     = "eastus"
}

variable "storage_account_prefix" {
  description = "Lowercase prefix for the storage account name. Azure storage account names must be globally unique, 3-24 chars, lowercase letters and numbers only."
  type        = string
  default     = "machinestore"

  validation {
    condition     = can(regex("^[a-z0-9]{3,16}$", var.storage_account_prefix))
    error_message = "storage_account_prefix must be 3-16 chars and contain only lowercase letters and numbers."
  }
}

variable "container_name" {
  description = "Name of the blob container to create."
  type        = string
  default     = "data"
}

variable "state_container_name" {
  description = "Name of the blob container used for OpenTofu state after bootstrap."
  type        = string
  default     = "tfstate"
}

variable "tags" {
  description = "Tags applied to Azure resources."
  type        = map(string)
  default = {
    project     = "machine"
    environment = "dev"
    managed_by  = "opentofu"
  }
}
