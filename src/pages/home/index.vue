<template>
  <div class="home-wrapper">
    <el-form class="home-form" label-width="80px">
      <el-form-item label="Hello">
        <el-input v-model="input" />
      </el-form-item>
      <el-form-item label="文件目录">
        <el-row>
          <el-col :span="18">
            <el-input v-model="destPath" placeholder="请选择文件目录" disabled />
          </el-col>
          <el-col :span="6">
            <el-button type="primary" @click="selectDestFolder">选择</el-button>
          </el-col>
        </el-row>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="writeFile">写文件到指定目录</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script>
import { getDesktopDir, writeFile } from '@/apis/home.js'
import { open } from '@tauri-apps/api/dialog'

export default {
  data() {
    return {
      input: "world",
      destPath: "",
    }
  },
  mounted() {
    this.getDesktopDir()
  },
  methods: {
    getDesktopDir() {
      getDesktopDir().then(res => {
        const { data } = JSON.parse(res)
        this.destPath = data
      })
    },
    selectDestFolder() {
      let openDirOption = {
        directory: true
      }
      if (this.destPath !== "") {
        openDirOption.defaultPath = this.destPath
      }
      open(openDirOption).then(res => {
        if (res) {
          this.destPath = res
        }
      })
    },
    writeFile() {
      const data = {
        content: "Hello " + this.input,
        dest_path: this.destPath
      }
      writeFile(JSON.stringify(data)).then(res => {
        const { code, data } = JSON.parse(res)
        if (code === 200) {
          this.$message(
            {
              message: 'Success',
              type: 'success'
            }
          )
        } else {
          this.$message.error(data)
        }
      })
    }
  }
}
</script>

<style lang="scss">
.home-wrapper {
  .home-form {
    padding-right: 50px;
  }
}
</style>