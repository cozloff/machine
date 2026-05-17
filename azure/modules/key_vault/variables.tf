variable "name" {
  description = "Optional explicit Key Vault name. If null, name_prefix plus a random suffix is used."
  type        = string
  default     = null

  validation {
    condition     = var.name == null || can(regex("^[a-zA-Z][a-zA-Z0-9-]{1,22}[a-zA-Z0-9]$", var.name))
    error_message = "name must be 3-24 characters, start with a letter, end with a letter or number, and contain only letters, numbers, and hyphens."
  }
}

variable "name_prefix" {
  description = "Prefix used when generating a Key Vault name. Key Vault names must be globally unique."
  type        = string

  validation {
    condition     = can(regex("^[a-zA-Z][a-zA-Z0-9-]{1,15}$", var.name_prefix))
    error_message = "name_prefix must be 2-16 characters, start with a letter, and contain only letters, numbers, and hyphens."
  }
}

variable "resource_group_name" {
  description = "Resource group name for the Key Vault."
  type        = string
}

variable "location" {
  description = "Azure region for the Key Vault."
  type        = string
}

variable "tenant_id" {
  description = "Azure AD tenant ID for the Key Vault."
  type        = string
}

variable "current_principal_id" {
  description = "Object ID granted Key Vault Administrator on the vault. Set null to skip assignment."
  type        = string
  default     = null
}

variable "sku_name" {
  description = "Key Vault SKU."
  type        = string
  default     = "standard"

  validation {
    condition     = contains(["standard", "premium"], var.sku_name)
    error_message = "sku_name must be standard or premium."
  }
}

variable "purge_protection_enabled" {
  description = "Whether purge protection is enabled. Recommended true outside throwaway environments."
  type        = bool
  default     = false
}

variable "soft_delete_retention_days" {
  description = "Soft delete retention in days."
  type        = number
  default     = 7

  validation {
    condition     = var.soft_delete_retention_days >= 7 && var.soft_delete_retention_days <= 90
    error_message = "soft_delete_retention_days must be between 7 and 90."
  }
}

variable "public_network_access" {
  description = "Whether public network access is enabled for the Key Vault."
  type        = bool
  default     = true
}

variable "network_acls" {
  description = "Optional Key Vault network ACL configuration. Leave null to omit network_acls."
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
    condition = var.network_acls == null || (
      contains(["AzureServices", "None"], var.network_acls.bypass) &&
      contains(["Allow", "Deny"], var.network_acls.default_action)
    )
    error_message = "network_acls.bypass must be AzureServices or None, and default_action must be Allow or Deny."
  }
}

variable "tags" {
  description = "Tags applied to the Key Vault."
  type        = map(string)
  default     = {}
}
