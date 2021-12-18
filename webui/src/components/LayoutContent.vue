<template>
    <div v-for="server in serverStatusList" :key="server.status.world">
        <n-card :title="server.status.world" hoverable>
            <n-space justify="space-between" align="center">
                <div
                    :style="{ color: server.status.playercount > 0 ? 'green' : 'gray' }"
                >在线人数：{{ server.status.playercount }}/{{ server.status.maxplayers }}</div>
                <n-button disabled>刷新</n-button>
            </n-space>
        </n-card>
    </div>
</template>

<script setup lang='ts'>
import { Ref, ref } from '@vue/reactivity';
import { NCard, NSpace, NButton } from 'naive-ui'

const serverStatusList: Ref<ServerStatus[]> = ref([])

async function getStatus() {
    try {
        const fetchRes = await fetch("/api/info?token=token1234")
        serverStatusList.value = await fetchRes.json() as ServerStatus[];
        console.log("fetch res:", serverStatusList.value)
    } catch (error) {
        console.log("fetch catch error:", error);
    }
}
getStatus()



interface ServerStatus {
    port: number,
    status: {
        status: string,
        name: string,
        serverversion: string,
        tshockversion: {
            Major: number,
            Minor: number,
            Build: number,
            Revision: number,
            MajorRevision: number,
            MinorRevision: number
        },
        port: number,
        playercount: number,
        maxplayers: number,
        world: string,
        uptime: string,
        serverpassword: boolean
    }
}

</script>

<style scoped>
.n-card {
    width: 350px;
    margin: 10px 0;
}
</style>