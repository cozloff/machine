# Azure Blob Storage With OpenTofu

This creates:

- one Azure resource group
- one StorageV2 account
- one private Blob container
- one private Blob container for OpenTofu state

## Prerequisites

Install:

- OpenTofu: <https://opentofu.org/docs/intro/install/>
- Azure CLI: <https://learn.microsoft.com/cli/azure/install-azure-cli>

Authenticate:

```bash
az login
az account set --subscription "<subscription-id-or-name>"
```

## Run

```bash
cd azure
cp terraform.tfvars.example terraform.tfvars
tofu init
tofu plan
tofu apply
```

## Move State Into Azure Blob

The Azure Blob backend cannot be fully self-referential on the first run because
OpenTofu initializes the backend before it creates resources. Bootstrap with
local state first, then migrate the local state into the blob container.

After the first `tofu apply` succeeds:

```bash
tofu output -raw storage_account_name
cp backend.tf.example backend.tf
```

Edit `backend.tf` and replace:

```text
<storage-account-name-from-output>
```

with the value from `tofu output -raw storage_account_name`.

Then migrate:

```bash
tofu init -migrate-state
```

After migration, future `tofu plan` and `tofu apply` runs use the Azure Blob
container for state.

## Destroy

```bash
tofu destroy
```

If the backend is enabled, `tofu destroy` removes the Azure resources but the
backend blob may still contain the final state object until you delete it
manually from the storage account.
