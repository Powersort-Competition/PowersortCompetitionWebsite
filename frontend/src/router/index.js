import {createRouter, createWebHashHistory} from "vue-router";
import HomeView from "../views/Home.vue";

const routes = [
    {
        path: "/",
        name: "home",
        component: HomeView,
    },
    {
        path: "/aboutA",
        name: "aboutA",
        // route level code-splitting
        // this generates a separate chunk (about.[hash].js) for this route
        // which is lazy-loaded when the route is visited.
        component: function () {
            return import(/* webpackChunkName: "about" */ "../views/AboutA.vue");
        },
    },
    {
        path: "/aboutB",
        name: "aboutB",
        component: function () {
            return import("../views/AboutB.vue");
        },
    },
    {
        path: "/aboutC",
        name: "aboutC",
        component: function () {
            return import("../views/AboutC.vue");
        },
    },
    {
        path: "/login",
        name: "login",
        component: function () {
            return import("../views/LogIn.vue");
        },
    },
    {
        path: "/trackA",
        name: "trackA",
        component: function () {
            return import("../views/TrackA.vue");
        },
    },
    {
        path: "/trackB",
        name: "trackB",
        component: function () {
            return import("../views/TrackB.vue");
        },
    },
    {
        path: "/trackC",
        name: "trackC",
        component: function () {
            return import("../views/TrackC.vue");
        }
    },
    {
        path: "/useful",
        name: "useful",
        component: function () {
            return import("../views/Useful.vue");
        },
    },
    {
        path: "/stats",
        name: "stats",
        component: function () {
            return import("../views/Stats.vue");
        },
    },
    {
        path: "/submission",
        name: "submission",
        component: function () {
            return import("../views/Submission.vue");
        },
    },
];

const router = createRouter({
    history: createWebHashHistory(),
    routes,
    scrollBehavior(to, from, savedPosition) {
        // always scroll to top
        // https://router.vuejs.org/guide/advanced/scroll-behavior.html
        return {top: 0, behavior: 'instant'}
    },
});

export default router;
