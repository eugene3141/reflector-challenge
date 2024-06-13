const routes = [
    {
        path: "/",
        name: "Layout",
        redirect: "/my_loans",
        component: () => import("@/Layout/index.vue"),
        children: [
            {
                path: "/my_loans",
                name: "My Loans",
                component: () => import("@/views/MyLoans/Index.vue"),
            },
            {
                path: "/new_loan",
                name: "New Loan",
                component: () => import("@/views/NewLoan/Index.vue"),
            },
            {
                path: "/claimable_balances",
                name: "Claimable Balances",
                component: () => import("@/views/ClaimableBalances/Index.vue"),
            },
            {
                path: "/loan/:id",
                name: "Loan",
                component: () => import("@/views/Loan/Index.vue"),
            },
        ],
    },
];

export default routes;
