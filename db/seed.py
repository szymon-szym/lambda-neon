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

parsed_movies = []

# read movies from movies.csv
with open('movies_sm.csv', 'r') as f:
    csv_reader = csv.reader(f)
    header = next(csv_reader)
    
    for movie in csv_reader:
        parsed_movies.append({
            'title': movie[0],
            'short_description': movie[4]
        })
    


print(parsed_movies[0])

# create client for bedrock embeddings model, use aws profile
bedrock = boto3.client(service_name='bedrock-runtime', region_name='us-east-1')

cur = conn.cursor()

counter = 0

for item in parsed_movies:

    counter += 1

    print('getting embeddings for item', counter)

    body = json.dumps({"inputText": item['short_description']})

    response = bedrock.invoke_model(body=body, modelId='amazon.titan-embed-text-v2:0')

    response_body = json.loads(response.get('body').read())
    
    embedding = response_body.get('embedding')
    
    item['embeddings'] = embedding

    cur.execute("INSERT INTO movies (title, short_description, embeddings) VALUES (%s, %s, %s)", (item['title'], item['short_description'], item['embeddings'])) 

conn.commit()




# def query_comps(query):

#     #open postgres connection
#     conn = psycopg2.connect(
#         host="localhost",
#         database="vectordb",
#         user="testuser",
#         password="testpwd",
#         port="5555")



#     cur = conn.cursor()

#     # create client for bedrock embeddings model, use aws profile
#     bedrock = boto3.client(service_name='bedrock-runtime', region_name='us-east-1')

#     body = json.dumps({"inputText": query})
#     response = bedrock.invoke_model(body=body, modelId='amazon.titan-embed-text-v2:0')

#     response_body = json.loads(response.get('body').read())

#     embedding = response_body.get('embedding')

#     # query postgres for closest match
#     cur.execute(f"""
#                 SELECT id, type, subtype, description
#                 FROM competencies
#                 ORDER BY embeddings <-> '{embedding}' LIMIT 3;
#                 """)
#     result = cur.fetchall()

#     # print(result)

#     # close postgres connection
#     cur.close()
#     conn.close()

#     print(result)

# query_comps('chcę poćwiczyć liczenie do 100')

# read_comps()