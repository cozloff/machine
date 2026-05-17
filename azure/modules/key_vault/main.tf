resource "random_string" "key_vault_suffix" {
  count = var.name == null ? 1 : 0

  length  = 6
  upper   = false
  special = false
}

locals {
  generated_name = var.name == null ? substr(
    lower(replace("${var.name_prefix}${random_string.key_vault_suffix[0].result}", "-", "")),
    0,
    24
  ) : null

  key_vault_name = coalesce(var.name, local.generated_name)
}

resource "azurerm_key_vault" "this" {
  name                          = local.key_vault_name
  location                      = var.location
  resource_group_name           = var.resource_group_name
  tenant_id                     = var.tenant_id
  sku_name                      = var.sku_name
  soft_delete_retention_days    = var.soft_delete_retention_days
  purge_protection_enabled      = var.purge_protection_enabled
  public_network_access_enabled = var.public_network_access
  enable_rbac_authorization     = true

  enabled_for_deployment          = true
  enabled_for_disk_encryption     = true
  enabled_for_template_deployment = true

  dynamic "network_acls" {
    for_each = var.network_acls == null ? [] : [var.network_acls]

    content {
      bypass                     = network_acls.value.bypass
      default_action             = network_acls.value.default_action
      ip_rules                   = network_acls.value.ip_rules
      virtual_network_subnet_ids = network_acls.value.virtual_network_subnet_ids
    }
  }

  tags = var.tags
}

resource "azurerm_role_assignment" "current_principal_key_vault_administrator" {
  count = var.current_principal_id == null ? 0 : 1

  scope                = azurerm_key_vault.this.id
  role_definition_name = "Key Vault Administrator"
  principal_id         = var.current_principal_id
}
