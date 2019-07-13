from graphqlclient import GraphQLClient

# pip3 install graphqlclient

client = GraphQLClient('http://localhost:3000/graphql')

query = '''
{
    human(id: "1000") {
    id
    name
  }
}
'''

result = client.execute(query)

print(result)


