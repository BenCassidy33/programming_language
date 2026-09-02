module shape;

use math;

interface Shape {
    float area(&self);
    float perimeter(&self);
}

struct Rectangle : Shape {
    float width;
    float height;
}

float Rectangle::area(&self) {
    self.width * self.height
}

float Rectangle::perimeter(&self) {
    (2 * self.width) + (2 * self.height)
}

struct Circle : Shape {
    float radius;

    float area(&self) {
        return math.PI * math.squared(self.radius);
    }

    inline float perimeter(&self) {
        return 2 * math.PI * self.radius;
    }
}

module Triangle {
    enum Kind {
        Equilateral,
        Isosceles,
        Scalene
    }

    struct Triangle {
        Kind kind;
        (float, float)[3] points;
    }

    impl Shape for Triangle {
        float area(&self) {
            ...
        }

        float perimeter(&self) {
            ...
        }
    }
}
