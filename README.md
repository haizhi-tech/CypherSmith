# CypherSmith

## Description
CypherSmith is a random cypher generator for OpenCypher. Its paragon is SQLsmith, a random SQL query generator.

Graph databases are widely used, but don't have any automated statement production tool to find bugs, so I wrote one myself.

It currently supports generating RegularQuery.


## Building

Release
```
$ cargo build --release
```

## Usage
CypherSmith connects to the target graph database to send the generated queries to. Beware that CypherSmith does call functions that could possibly have side-effects (e.g. UpdatingClause). Use a suitably underprivileged user for its connection to avoid this.


Example Invocations:
```
# generate queries instead of executing them
$ cypher-smith --schema ./test/schema.json --config ./test/config.json

# testing AtlasGraph
$ cypher-smith --schema ./test/schema.json --config ./test/atlas_config.json --atlas ./test/atlas.json
```


## Description of the input JSON files:
CypherSmith can be configured easily via configuration files, which are described below. The user can modify schema to configure Schema Information. A sample of schema.json is shown below. Some comments are added to provide a brief explanation.


The following options are currently supported:
|  option   |  description |
|  :----:  | :----:  |
|  --schema | Schema information for the graph model  |
|  --config | Basic tool configuration |
|  --atlas(Option) | Connection configuration for AtlasGraph |

### Schema Information description
#### Schema Config 
| option | description |
| :----: | :----: |
| --name | Graph name |
| --vertex_labels | Vertex Label Information: LabelName, LabelId, LabelKind, Properties|
| --edge_labels | Edge Label Information: LabelName, LabelId, LabelKind, Properties |

```
{
    // graph name
    "name": "cyphersmith",
    // vertex labels
    "vertex_labels": [
        {
            // vertex label name
            "label_name": "Person",
            // vertex label id, user define
            "label_id": 0,
            // kind: vertex or edge.
            "kind": "Vertex",
            // properties: name(String), propr_id(user define, int32), prop type, is_pk(bool), nullable(bool), is_delete(bool)
            // prop_type: "Int32" | "Null" | "Bool" | "Int64" | "Float" | "Double" | "Date" | "DateTime" | "String"
            "properties": [
                {
                    "name": "id",
                    "prop_id": 0,
                    "prop_type": "Int32",
                    "is_pk": true,
                    "nullable": false,
                    "is_delete": false
                }
            ]
        },
    ],
    "edge_labels": [
        {
            "label_name": "Knows",
            "label_id": 3, 
            "kind": { 
                // Edge: relations: [[source vertex_id(user define before),  dst vertex_id]]
                // is_directed: whether directed or not
                "Edge": {
                    "relations": [[0,0]],
                    "is_directed": true
                }
            },
            "properties":[
                {
                    "name": "edge_id",
                    "prop_id": 0,
                    "prop_type": "Int32",
                    "is_pk": false,
                    "nullable": false,
                    "is_delete": false
                }
            ]
        }
    ]
}
```

Example: ./test/schema.json

### Detailed configuration description

#### Basic Config
|  option   |  description |
|  :----:  | :----:  |
|  --call_query | whether generate function call cypher string(WIP) |
|  --max_queries | terminate after generating this many queries |
|  --dry_run | print queries instead of executing them |
| --dry_run_path(Option) | Cypehr Output file path |
| --verbose(Option) | emit progress output |
| --dump_all_graphs | dump generated ASTs for debugging |

Example: ./test/config.json ./test/atlas_config.json

```
{
    "call_query": false,
    "max_queries": 1000,
    "dry_run": true,
    "verbose": "./report/test.csv",
    "dump_all_graphs": false
}
```

#### AtlasConnection Config
Configures the user, password, and address information for AtlasGraph Server.
| option | description |
| :----: | :----: |
| --address | ip:port |
| --username | UserName |
| --password | Password |

Example: ./test/atlas.json