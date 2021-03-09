extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::{Region};
use rusoto_s3::{S3Client, S3, GetPublicAccessBlockRequest};

#[tokio::main]
async fn main() {
  let client = S3Client::new(Region::EuCentral1);

  match client.list_buckets().await {
    Ok(bucket_output) => match bucket_output.buckets {
      Some(bucket_list) => {

        for mut bucket in bucket_list {
          let mut get_public_access_block_request = GetPublicAccessBlockRequest { bucket: bucket.name.unwrap(), ..Default::default() };

          match client.get_public_access_block(get_public_access_block_request).await {
            Ok(bucket_access_output) => match bucket_access_output.public_access_block_configuration {
              Some(access_block) => {
                println!("block_public_acls: {}", access_block.block_public_acls.unwrap());
                println!("block_public_policy: {}", access_block.block_public_policy.unwrap());
                println!("ignore_public_acls: {}", access_block.ignore_public_acls.unwrap());
                println!("restrict_public_buckets: {}", access_block.restrict_public_buckets.unwrap())
              }
              None => println!("No bucket access policy found")
            },
            _ => println!("Didn't find any access policy for bucket {}", bucket.name.unwrap())
          }
        }
      }
      None => println!("No buckets found")
    },
    Err(error) => {
      println!("Error: {:?}", error);
    }
  }


}
