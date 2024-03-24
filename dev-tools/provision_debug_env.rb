# frozen_string_literal: true

require 'aws-sdk'

dynamo_port = 8000
aws_iam_key_id = 'EkikaAdmin'
aws_iam_secret_key = 'EkikaAdmin'
region = 'ap-northeast-1'

credentials = Aws::Credentials.new(aws_iam_key_id, aws_iam_secret_key)
ddb = Aws::DynamoDB::Client.new(region: region, credentials: credentials,
                                endpoint: format('http://localhost:%d', dynamo_port))

# create user table
begin
  ddb.describe_table({ table_name: 'users' })
rescue StandardError
  users_table_def = {
    table_name: 'users',
    attribute_definitions: [
      {
        attribute_name: 'Id',
        attribute_type: 'S'
      }
    ],
    key_schema: [
      {
        attribute_name: 'Id',
        key_type: 'HASH'
      }
    ],
    provisioned_throughput: {
      read_capacity_units: 5,
      write_capacity_units: 5
    }
  }
  ddb.create_table(users_table_def)
end

admin_user = {
  item: {
    'Id' => 'admin',
    'PreferredUserName': 'namachan10777',
    'Name' => 'namachan10777',
    'Summary' => 'Administrator on this server',
    'Icon' => [
      'https://www.namachan10777.dev/icon.webp'
    ],
    'Kind' => 'Person'
  },
  return_consumed_capacity: 'TOTAL',
  table_name: 'users'
}

ddb.put_item(admin_user)
