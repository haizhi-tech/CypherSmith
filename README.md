# Cypher-smith
Random Cypher Generator of OpenCypher


## Building

Release
```
$ cargo build --release
```

## Usage

Example Invocations:
```
# generate queries instead of executing them
$ cypher-smith --schema ./test/schema.json --config ./test/config.json

# testing AtlasGraph
$ cypher-smith --schema ./test/schema.json --config ./test/atlas_config.json --atlas ./test/atlas.json
```


## Description of the input JSON files:

The following options are currently supported:
|  option   |  description |
|  :----:  | :----:  |
|  --schema | Schema information for the graph model  |
|  --config | Basic tool configuration |
|  --atlas(Option) | Connection configuration for AtlasGraph |

### Detailed configuration description

#### Basic Config
|  option   |  description |
|  :----:  | :----:  |
|  --call_query | whether generate function call cypher string |
|  --max_queries | terminate after generating this many queries |
|  --dry_run | print queries instead of executing them |
| --dry_run_path(Option) | Cypehr Output file path |
| --verbose(Option) | emit progress output |
| --dump_all_graphs | dump generated ASTs for debugging |

Example: ./test/config.json ./test/atlas_config.json

#### Schema Config 
| option | description |
| :----: | :----: |
| --name | Graph name |
| --vertex_labels | Vertex Label Information: LabelName, LabelId, LabelKind, Properties|
| --edge_labels | Edge Label Information: LabelName, LabelId, LabelKind, Properties |

Example: ./test/schema.json

#### AtlasConnection Config
Configures the user, password, and address information for AtlasGraph Server.
| option | description |
| :----: | :----: |
| --address | ip:port |
| --username | UserName |
| --password | Password |

Example: ./test/atlas.json