terraform {
  backend "azurerm" {
    resource_group_name  = "rg-machine-dev"
    storage_account_name = "machinestorerbh0w0dx"
    container_name       = "tfstate"
    key                  = "machine.azure.tfstate"
  }
}
