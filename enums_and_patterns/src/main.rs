mod enums;
mod patterns;

fn main() {

    use std::mem::size_of;
    use std::cmp::Ordering;

    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<enums::HttpStatus>(), 2); //404 doesn't fit in a u8.

    let four_score_seven_years_ago =
        enums::RoughTime::InThePast(enums::TimeUnit::Years, 4*20 + 7);

    let three_hours_from_now =
        enums::RoughTime::InTheFuture(enums::TimeUnit::Hours, 3);

    let unit_sphere = enums::Shape::Sphere {
        center: enums::Point3d::ORIGIN,
        radius: 1.0,
    };

    //Binary trees
    let jupiter_tree = enums::BinaryTree::NonEmpty(Box::new(enums::TreeNode {
        element: "Jupiter",
        left: enums::BinaryTree::Empty,
        right: enums::BinaryTree::Empty,
    }));

    let mars_tree = enums::BinaryTree::NonEmpty(Box::new(enums::TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: enums::BinaryTree::Empty,
    }));

    let tree = enums::BinaryTree::NonEmpty(Box::new(enums::TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: enums::BinaryTree::Empty
    }));

    let mut tree = enums::BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Venus")

}

/// Casting a C-style enum is allowed. However, casting in the other direction, from the integer to
/// the enum, is not.
fn http_status_from_u32(n:u32) -> Option<enums::HttpStatus> {
    match n {
        200 => Some(enums::HttpStatus::Ok),
        304 => Some(enums::HttpStatus::NotModified),
        404 => Some(enums::HttpStatus::NotFound),
        _ => None
    }
}

