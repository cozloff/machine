terraform {
  required_version = ">= 1.6.0"

  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 4.0"
    }

    random = {
      source  = "hashicorp/random"
      version = "~> 3.6"
    }
  }
}

provider "azurerm" {
  features {}
}

data "azurerm_client_config" "current" {}

module "resource_group" {
  source = "./modules/resource_group"

  name     = var.resource_group_name
  location = var.location
  tags     = var.tags
}

module "storage" {
  source = "./modules/storage"

  resource_group_name    = module.resource_group.name
  location               = module.resource_group.location
  storage_account_prefix = var.storage_account_prefix
  container_name         = var.container_name
  state_container_name   = var.state_container_name
  tags                   = var.tags
}

module "key_vault" {
  source = "./modules/key_vault"

  name                       = var.key_vault_name
  name_prefix                = var.key_vault_name_prefix
  resource_group_name        = module.resource_group.name
  location                   = module.resource_group.location
  tenant_id                  = data.azurerm_client_config.current.tenant_id
  current_principal_id       = data.azurerm_client_config.current.object_id
  sku_name                   = var.key_vault_sku_name
  purge_protection_enabled   = var.key_vault_purge_protection_enabled
  soft_delete_retention_days = var.key_vault_soft_delete_retention_days
  public_network_access      = var.key_vault_public_network_access
  network_acls               = var.key_vault_network_acls
  tags                       = var.tags
}
