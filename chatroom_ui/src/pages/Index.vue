<template>
  <q-page class="row items-center justify-center content-center">
    <q-card>
      <q-form ref="form" @submit.prevent="doSubmit">
        <q-card-section>
          <q-input
            v-model="name"
            label="請輸入名稱"
            :rules="[(v) => (!!v && v.length >= 4) || '至少四個字']"
          />
        </q-card-section>
        <q-card-actions>
          <q-btn type="submit" label="確定" color="primary" flat />
        </q-card-actions>
      </q-form>
    </q-card>
  </q-page>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { QForm } from 'quasar';
import { useStore } from 'src/store';

//////////////// 內部共用工具
const _store = useStore();
const _router = useRouter();

//////////////// $refs
const form = ref<QForm>();

//////////////// 名稱及相關處理
const name = ref('');
async function doSubmit() {
  if (!(await form.value?.validate(true))) return;

  await _store.dispatch('setName', name.value);
  await _router.push({ name: 'Chat' });
}
</script>
