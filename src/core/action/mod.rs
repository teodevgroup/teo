pub(crate) mod actual;
pub(crate) mod intent;
pub(crate) mod amount;
pub(crate) mod source;
pub(crate) mod position;

use self::actual::ActionActual;
use self::intent::ActionIntent;
use self::amount::ActionAmount;
use self::source::ActionSource;
use self::position::ActionPosition;

pub(crate) struct Action {
    intent: ActionIntent,
    actual: ActionActual,
    position: ActionPosition,
    amount: ActionAmount,
    source: ActionSource,
}
