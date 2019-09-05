

struct Vector3<T>{
    x:T,
    y:T,
    z:T,
}



impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T>{
        Vector3{
            x, y, z,
        }
    }
}

struct HalfEdgeVertex<T>{
    position: Vector3<T>,
    normal:  Vector3<T>,
    edge: *mut HalfEdge<T> // one of the half-edges emantating from the vertex
}


struct HalfEdgeFace<T>{
    edge: *mut HalfEdge<T>// one of the half-edges bordering the face 
}

impl<T> HalfEdgeFace<T>{
    pub fn visit_edge_around_face(&self, visitor: fn(&HalfEdge<T>)){
        unsafe{
            let edge = &*self.edge;
            visitor(edge);
            loop{
                let next_edge = edge.next();
                if next_edge as *const HalfEdge<T> != edge as *const HalfEdge<T>{
                    visitor(next_edge);
                } else {
                    break
                }
            }
        }
    }
}

// http://www.flipcode.com/archives/The_Half-Edge_Data_Structure.shtml
struct HalfEdge<T>{
    vert: *mut HalfEdgeVertex<T>,
    pair: *mut HalfEdge<T>,
    face: *mut HalfEdgeFace<T>,
    next: *mut HalfEdge<T>
}

impl<T> HalfEdge<T>{
    pub fn next(&self) -> &HalfEdge<T>{
        unsafe{
            &*self.next
        }
    }
}


struct HalfEdgeMesh<T>{
    HalfEdges: Vec<HalfEdge<T>>
}

trait EditableMesh {

}


fn main() {
    println!("Hello, world!");
}
