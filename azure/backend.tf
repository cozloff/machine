terraform {
  backend "azurerm" {
    resource_group_name  = "rg-machine-dev"
    storage_account_name = "machinestore47ma0p84"
    container_name       = "tfstate"
    key                  = "machine.azure.tfstate"
  }
}
