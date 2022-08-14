import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const state = {
  foo: "bar"
}

const mutations = {
  setFoo(state, bar) {
    state.foo = bar
  }
}

const actions = {
  setFoo({ commit }, bar) {
    commit('setFoo', bar)
  }
}

const store = new Vuex.Store({
  state,
  mutations,
  actions
})

export default store
