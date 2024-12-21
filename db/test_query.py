import json
import boto3
from urllib.parse import urlparse
import os
from dotenv import load_dotenv
import csv

import psycopg2

load_dotenv()

database_url = os.getenv('DATABASE_URL')

parsed_url = urlparse(database_url)

conn = psycopg2.connect(
    host=parsed_url.hostname,
    database=parsed_url.path[1:],  # Remove leading slash from path
    user=parsed_url.username,
    password=parsed_url.password,
    port=parsed_url.port or 5432  # Default to 5432 if port not specified
)

print("Connected to PostgreSQL database")

def test_query(q):
    cur = conn.cursor()

    # create client for bedrock embeddings model, use aws profile
    bedrock = boto3.client(service_name='bedrock-runtime', region_name='us-east-1')

    body = json.dumps({"inputText": q})
    response = bedrock.invoke_model(body=body, modelId='amazon.titan-embed-text-v2:0')

    response_body = json.loads(response.get('body').read())

    embedding = response_body.get('embedding')

    # query postgres for closest match
    cur.execute(f"""
                SELECT id, title, short_description, embeddings <+> '{embedding}' as distance
                FROM movies
                ORDER BY embeddings <+> '{embedding}' 
                LIMIT 5;
                """)
    result = cur.fetchall()

    # print(result)

    # close postgres connection
    cur.close()
    conn.close()

    print(result)

test_query('dinozaury')
