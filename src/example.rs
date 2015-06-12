#include "vp.h"
#include <math.h>
#include <stdio.h>

typedef struct point {float x,y;} point;

float get_distance(const void *a, const void *b) {
    float dx = ((point *)a)->x - ((point *)b)->x;
    float dy = ((point *)a)->y - ((point *)b)->y;

    return sqrt(dx*dx + dy*dy); // sqrt is required
}

int main(void) {

    // The library operates on pointers to elements
    point *points[] = {&(point){2,3}, &(point){0,1}, &(point){4,5}};

    vp_handle *vp = vp_init((void**)points, 3, get_distance);

    int index = vp_find_nearest(vp, &(point){1,2});

    printf("The nearest point is at (%.0f,%.0f)\n", points[index]->x, points[index]->y);

    vp_free(vp);
}