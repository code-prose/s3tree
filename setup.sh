source .dev.sh
aws s3 mb s3://test-bucket

aws s3 cp ./example-nested.csv s3://test-bucket/nested/example-nested.csv
aws s3 cp ./example.txt s3://test-bucket/example.txt
