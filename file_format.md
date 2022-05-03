## Maikor Game File Format

### Order

* Header
* Name
* Author
* Version
* Atlas blocks lengths
* Main Code
* Code Banks (optional)
* Atlas Banks (optional)

### Header
| Byte Offset | Hex Offset | Length | Note                        |
|-------------|------------|--------|-----------------------------|
| 0           | 0x0        | 2      | Header                      |
| 2           | 0x2        | 1      | Maikor File Format Version  |
| 3           | 0x3        | 2      | Min. Maikor Version         |
| 5           | 0x5        | 2      | Compiled for Maikor Version |
| 7           | 0x7        | 4      | Program ID                  |
| 11          | 0xB        | 2      | Build                       |
| 13          | 0xD        | 1      | Version Length              |
| 14          | 0xE        | 1      | Name Length                 |
| 15          | 0xF        | 1      | Author Length               |
| 16          | 0x10       | 1      | Code Bank Count             | 
| 17          | 0x11       | 1      | RAM Bank Count              | 
| 18          | 0x12       | 1      | Atlas Bank Count            |

### Main Code

8192 bytes that will be copied to $0 in VM memory.

### Code banks

`Code Bank Count` blocks of 8192 bytes

### Atlas blocks lengths

List of words (2 bytes) (`Atlas Bank Count` long), each being the length of a atlas bank

### Atlas banks

Followed by the atlas blocks
