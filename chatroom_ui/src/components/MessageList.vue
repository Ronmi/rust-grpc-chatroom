<template>
  <q-list>
    <q-item v-for="(msg, idx) in items" :key="idx">
      <q-item-section top>
        <div class="row justify-start content-top items-top">
          <q-chip dense size="sm" color="primary" outline square>{{
            msg.name
          }}</q-chip>
          <div>{{ msg.msg }}</div>
        </div>
      </q-item-section>
      <q-item-section side>
        {{ fmtTime(msg.createAt) }}
      </q-item-section>
    </q-item>
  </q-list>
</template>

<script lang="ts" setup>
import { defineProps } from 'vue';
import { Message } from 'src/proto/chat_pb';

//////////////// props
defineProps<{ items: Message.AsObject[] }>();

//////////////// 時間顯示的處理
function _pad(n: number): string {
  const ret = String(n);
  if (ret.length < 2) {
    return '0'.repeat(2 - ret.length) + ret;
  }

  return ret;
}
function fmtTime(ts: number): string {
  const d = new Date(ts * 1000);
  return _pad(d.getHours()) + ':' + _pad(d.getSeconds());
}
</script>
