/**
 * Node - a node of in the singly linked list.
 *
 * @next: next node.
 * @data: data stored in the next node.
 *
 * Description - this node links with other nodes to create a singly
 * linked list.
 */

struct Node {
    next: &Node, //is throwing an error will work on it
    data: String
}

/**
 * create_node - create a node.
 *
 * @data : Data to be added on the created node.
 *
 * Return: node created that points to null.
 */

fn create_node(data: String) -> Node {
    Node {
        next: None,
        data
    }
}













