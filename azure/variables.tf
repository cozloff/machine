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

variable "key_vault_name" {
  description = "Optional explicit Key Vault name. If null, key_vault_name_prefix plus a random suffix is used."
  type        = string
  default     = null

  validation {
    condition     = var.key_vault_name == null || can(regex("^[a-zA-Z][a-zA-Z0-9-]{1,22}[a-zA-Z0-9]$", var.key_vault_name))
    error_message = "key_vault_name must be 3-24 characters, start with a letter, end with a letter or number, and contain only letters, numbers, and hyphens."
  }
}

variable "key_vault_name_prefix" {
  description = "Prefix used when generating a Key Vault name. Key Vault names must be globally unique."
  type        = string
  default     = "kvmachine"

  validation {
    condition     = can(regex("^[a-zA-Z][a-zA-Z0-9-]{1,15}$", var.key_vault_name_prefix))
    error_message = "key_vault_name_prefix must be 2-16 characters, start with a letter, and contain only letters, numbers, and hyphens."
  }
}

variable "key_vault_sku_name" {
  description = "Key Vault SKU."
  type        = string
  default     = "standard"

  validation {
    condition     = contains(["standard", "premium"], var.key_vault_sku_name)
    error_message = "key_vault_sku_name must be standard or premium."
  }
}

variable "key_vault_purge_protection_enabled" {
  description = "Whether purge protection is enabled for the Key Vault. Recommended true outside throwaway environments."
  type        = bool
  default     = false
}

variable "key_vault_soft_delete_retention_days" {
  description = "Soft delete retention in days for the Key Vault."
  type        = number
  default     = 7

  validation {
    condition     = var.key_vault_soft_delete_retention_days >= 7 && var.key_vault_soft_delete_retention_days <= 90
    error_message = "key_vault_soft_delete_retention_days must be between 7 and 90."
  }
}

variable "key_vault_public_network_access" {
  description = "Whether public network access is enabled for the Key Vault."
  type        = bool
  default     = true
}

variable "key_vault_network_acls" {
  description = "Key Vault network ACL configuration. Set null to omit network_acls."
  type = object({
    bypass                     = string
    default_action             = string
    ip_rules                   = list(string)
    virtual_network_subnet_ids = list(string)
  })
  default = {
    bypass                     = "AzureServices"
    default_action             = "Allow"
    ip_rules                   = []
    virtual_network_subnet_ids = []
  }

  validation {
    condition = var.key_vault_network_acls == null || (
      contains(["AzureServices", "None"], var.key_vault_network_acls.bypass) &&
      contains(["Allow", "Deny"], var.key_vault_network_acls.default_action)
    )
    error_message = "key_vault_network_acls.bypass must be AzureServices or None, and default_action must be Allow or Deny."
  }
}
