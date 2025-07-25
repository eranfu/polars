"""
# --8<-- [start:read_parquet]
import polars as pl

source = "s3://bucket/*.parquet"

df = pl.read_parquet(source)
# --8<-- [end:read_parquet]

# --8<-- [start:scan_parquet_query]
import polars as pl

source = "s3://bucket/*.parquet"

df = pl.scan_parquet(source).filter(pl.col("id") < 100).select("id","value").collect()
# --8<-- [end:scan_parquet_query]


# --8<-- [start:scan_parquet_storage_options_aws]
import polars as pl

source = "s3://bucket/*.parquet"

storage_options = {
    "aws_access_key_id": "<secret>",
    "aws_secret_access_key": "<secret>",
    "aws_region": "us-east-1",
}
df = pl.scan_parquet(source, storage_options=storage_options).collect()
# --8<-- [end:scan_parquet_storage_options_aws]

# --8<-- [start:credential_provider_class]
lf = pl.scan_parquet(
    "s3://.../...",
    credential_provider=pl.CredentialProviderAWS(
        profile_name="..."
        assume_role={
            "RoleArn": f"...",
            "RoleSessionName": "...",
        }
    ),
)

df = lf.collect()
# --8<-- [end:credential_provider_class]

# --8<-- [start:credential_provider_custom_func]
def get_credentials() -> pl.CredentialProviderFunctionReturn:
    expiry = None

    return {
        "aws_access_key_id": "...",
        "aws_secret_access_key": "...",
        "aws_session_token": "...",
    }, expiry


lf = pl.scan_parquet(
    "s3://.../...",
    credential_provider=get_credentials,
)

df = lf.collect()
# --8<-- [end:credential_provider_custom_func]

# --8<-- [start:credential_provider_custom_func_azure]
def credential_provider():
    credential = DefaultAzureCredential(exclude_managed_identity_credential=True)
    token = credential.get_token("https://storage.azure.com/.default")

    return {"bearer_token": token.token}, token.expires_on


pl.scan_parquet(
    "abfss://...@.../...",
    credential_provider=credential_provider,
)

# Note that for the above case, this shortcut is also available:

pl.scan_parquet(
    "abfss://...@.../...",
    credential_provider=pl.CredentialProviderAzure(
        credentials=DefaultAzureCredential(exclude_managed_identity_credential=True)
    ),
)

# --8<-- [end:credential_provider_custom_func_azure]

# --8<-- [start:scan_pyarrow_dataset]
import polars as pl
import pyarrow.dataset as ds

dset = ds.dataset("s3://my-partitioned-folder/", format="parquet")
(
    pl.scan_pyarrow_dataset(dset)
    .filter(pl.col("foo") == "a")
    .select(["foo", "bar"])
    .collect()
)
# --8<-- [end:scan_pyarrow_dataset]

# --8<-- [start:write_parquet]
import polars as pl

df = pl.DataFrame(
    {
        "foo": ["a", "b", "c", "d", "d"],
        "bar": [1, 2, 3, 4, 5],
    }
)

destination = "s3://bucket/my_file.parquet"

df.write_parquet(destination)

# --8<-- [end:write_parquet]

# --8<-- [start:write_file_object]
import polars as pl
import s3fs
import gzip

df = pl.DataFrame(
    {
        "foo": ["a", "b", "c", "d", "d"],
        "bar": [1, 2, 3, 4, 5],
    }
)

destination = "s3://bucket/my_file.csv.gz"

fs = s3fs.S3FileSystem()

with fs.open(destination, "wb") as cloud_f:
    with gzip.open(cloud_f, "w") as f:
        df.write_csv(f)
# --8<-- [end:write_file_object]
"""
