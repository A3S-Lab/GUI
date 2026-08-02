import { RevisionActionRegistryV1 } from "./action-registry.ts";
import type { A3sApplicationHostV1 } from "./application.ts";
import { A3sClientSessionV1 } from "./client-session.ts";
import type { CompiledA3sFrameV1 } from "./frame.ts";

interface PreparedApplicationReplayV1 {
  readonly actions: RevisionActionRegistryV1;
  readonly session: A3sClientSessionV1;
}

/** Replays one retained committed frame into revision 1 of a fresh session. */
export async function prepareApplicationReplayV1(
  host: A3sApplicationHostV1,
  compiled: CompiledA3sFrameV1,
  previousSessionId: string,
): Promise<PreparedApplicationReplayV1> {
  validateReplayHost(host);
  const session = host.session ?? new A3sClientSessionV1(host.welcome);
  if (host.session !== undefined && host.session.welcome !== host.welcome) {
    throw new TypeError("recovery host session does not match its welcome message");
  }
  if (session.state.sessionId === previousSessionId) {
    throw new TypeError("application recovery requires a fresh host session identity");
  }
  const actions = new RevisionActionRegistryV1();
  const replayRevision = 1;
  try {
    actions.stage(replayRevision, compiled);
    const candidate = session.createRender(replayRevision, compiled.frame);
    const committed = await host.submitRender(candidate);
    session.commitRender(committed, actions);
    return Object.freeze({ actions, session });
  } catch (cause) {
    if (actions.state.pending?.renderRevision === replayRevision) {
      actions.reject(replayRevision);
    }
    if (session.state.pendingRenderRevision === replayRevision) {
      session.rejectRender(replayRevision);
    }
    throw cause;
  }
}

function validateReplayHost(host: A3sApplicationHostV1): void {
  if (
    typeof host !== "object" ||
    host === null ||
    typeof host.submitRender !== "function"
  ) {
    throw new TypeError("application recovery requires a typed host with submitRender");
  }
}
