<template>
    <div>
        <div class="topnav-wrapper">
            <div class="topnav-logo-wrapper">
                <router-link to="/">
                    <img src="@/assets/logo.png" class="logo" />  
                </router-link>
            </div>
            <div class="right-nav-item-wrapper">
                <div class="nav-item">关于</div>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    data() {
        return {
        };
    },
    mounted() {
    },
    methods: {
    }
};
</script>

<style lang="scss">
.topnav-wrapper {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 100;
    background-color: rgb(245, 245, 245);
    height: 50px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: #555;

    .topnav-logo-wrapper {
        display: flex;
        align-items: center;

        a {
            padding: 0 20px;
            display: inline-block;
            width: 30px;
            height: 30px;
        }

        .logo {
            width: 30px;
            height: 30px;
            line-height: 30px;
        }
    }

    .right-nav-item-wrapper {
        display: flex;
        align-items: center;

        .nav-item {
            height: 36px;
            line-height: 36px;
            border-radius: 17px;
            padding: 0 20px;
            cursor: pointer;
        }
    }
}
</style>
