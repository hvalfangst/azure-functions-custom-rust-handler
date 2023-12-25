terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "3.49.0"
    }
  }
}

provider "azurerm" {
  features {}
}

resource "azurerm_resource_group" "hvalfangst" {
  name     = "hvalfangstresourcegroup"
  location = "West Europe"
}

resource "azurerm_storage_account" "hvalfangst" {
  name                     = "hvalfangststorageaccount"
  resource_group_name      = azurerm_resource_group.hvalfangst.name
  location                 = azurerm_resource_group.hvalfangst.location
  account_tier             = "Standard"
  account_replication_type = "LRS"
}

resource "azurerm_service_plan" "hvalfangst" {
  name = "hvalfangstserviceplan"
  location = azurerm_resource_group.hvalfangst.location
  resource_group_name = azurerm_resource_group.hvalfangst.name
  os_type = "Linux"
  sku_name = "Y1"
}

resource "azurerm_linux_function_app" "example" {
  name                = "hvalfangstlinuxfunctionapp"
  resource_group_name = azurerm_resource_group.hvalfangst.name
  location            = azurerm_resource_group.hvalfangst.location
  storage_account_name       = azurerm_storage_account.hvalfangst.name
  storage_account_access_key = azurerm_storage_account.hvalfangst.primary_access_key
  service_plan_id            = azurerm_service_plan.hvalfangst.id
  site_config {
    application_stack{
      use_custom_runtime = true
    }
  }
}