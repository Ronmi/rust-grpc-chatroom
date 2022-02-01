<template>
  <q-page padding>
    <q-card flat>
      <q-card-section>
        <message-list :items="msgs" />
      </q-card-section>

      <q-card-section>
        <q-form @submit.prevent="say" class="row q-gutter-xs">
          <q-input
            v-model="myMsg"
            color="primary"
            class="col"
            :prefix="name + ':'"
            autofocus
            dense
            hide-hint
          />
          <q-btn type="submit" color="primary" class="col-auto" label="送出" />
        </q-form>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue';
import { useStore } from 'src/store';
import MessageList from 'src/components/MessageList.vue';
import { JoinReq, Message, NewMsgReq, SayReq } from 'src/proto/chat_pb';
import { ChatClient } from 'src/proto/ChatServiceClientPb';

//////////////// 共用資料
const _store = useStore();
const name = computed(() => _store.state.name);
const _api = new ChatClient('http://127.0.0.1:50051');

(function () {
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-explicit-any, @typescript-eslint/no-unsafe-assignment
  const x = (window as any).__GRPCWEB_DEVTOOLS__;
  if (x) {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-call
    x([_api]);
  }
})();

//////////////// 先抓舊資料
const msgs = ref<Message.AsObject[]>([]);
async function _boot() {
  const res = await _api.join(new JoinReq(), null);
  msgs.value = res.getMsgsList().map((v) => v.toObject());
}
_boot().then(null, null);

//////////////// 訂閱即時訊息通知
function _subscribe() {
  _api
    .newMsg(new NewMsgReq())
    .on('data', (v: Message) => {
      msgs.value.push(v.toObject());
    })
    .on('error', _subscribe);
}
_subscribe();

//////////////// 處理發言
const myMsg = ref('');
async function say() {
  await _api.say(new SayReq().setName(name.value).setMsg(myMsg.value), null);
  myMsg.value = '';
}
</script>
