#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pid: Pid,
    cccfg: Cccfg,
    _reserved2: [u8; 0x01f8],
    qchmapn: Qchmapn,
    _reserved3: [u8; 0x3c],
    dmaqnumn: Dmaqnumn,
    _reserved4: [u8; 0x1c],
    qdmaqnum: Qdmaqnum,
    _reserved5: [u8; 0x1c],
    quetcmap: Quetcmap,
    quepri: Quepri,
    _reserved7: [u8; 0x78],
    emr: Emr,
    emrh: Emrh,
    emcr: Emcr,
    emcrh: Emcrh,
    qemr: Qemr,
    qemcr: Qemcr,
    ccerr: Ccerr,
    ccerrclr: Ccerrclr,
    eeval: Eeval,
    _reserved16: [u8; 0x1c],
    draem: Draem,
    draehm: Draehm,
    _reserved18: [u8; 0x38],
    qraen: Qraen,
    _reserved19: [u8; 0x7c],
    qne0: Qne0,
    qne1: Qne1,
    qne2: Qne2,
    qne3: Qne3,
    qne4: Qne4,
    qne5: Qne5,
    qne6: Qne6,
    qne7: Qne7,
    qne8: Qne8,
    qne9: Qne9,
    qne10: Qne10,
    qne11: Qne11,
    qne12: Qne12,
    qne13: Qne13,
    qne14: Qne14,
    qne15: Qne15,
    _reserved35: [u8; 0x01c0],
    qstatn: Qstatn,
    _reserved36: [u8; 0x1c],
    qwmthra: Qwmthra,
    _reserved37: [u8; 0x1c],
    ccstat: Ccstat,
    _reserved38: [u8; 0xbc],
    aetctl: Aetctl,
    aetstat: Aetstat,
    aetcmd: Aetcmd,
    _reserved41: [u8; 0x08f4],
    er: Er,
    erh: Erh,
    ecr: Ecr,
    ecrh: Ecrh,
    esr: Esr,
    esrh: Esrh,
    cer: Cer,
    cerh: Cerh,
    eer: Eer,
    eerh: Eerh,
    eecr: Eecr,
    eecrh: Eecrh,
    eesr: Eesr,
    eesrh: Eesrh,
    ser: Ser,
    serh: Serh,
    secr: Secr,
    secrh: Secrh,
    _reserved59: [u8; 0x08],
    ier: Ier,
    ierh: Ierh,
    iecr: Iecr,
    iecrh: Iecrh,
    iesr: Iesr,
    iesrh: Iesrh,
    ipr: Ipr,
    iprh: Iprh,
    icr: Icr,
    icrh: Icrh,
    ieval: Ieval,
    _reserved70: [u8; 0x04],
    qer: Qer,
    qeer: Qeer,
    qeecr: Qeecr,
    qeesr: Qeesr,
    qser: Qser,
    qsecr: Qsecr,
    _reserved76: [u8; 0x0f68],
    er_rn: ErRn,
    erh_rn: ErhRn,
    ecr_rn: EcrRn,
    ecrh_rn: EcrhRn,
    esr_rn: EsrRn,
    esrh_rn: EsrhRn,
    cer_rn: CerRn,
    cerh_rn: CerhRn,
    eer_rn: EerRn,
    eerh_rn: EerhRn,
    eecr_rn: EecrRn,
    eecrh_rn: EecrhRn,
    eesr_rn: EesrRn,
    eesrh_rn: EesrhRn,
    ser_rn: SerRn,
    serh_rn: SerhRn,
    secr_rn: SecrRn,
    secrh_rn: SecrhRn,
    _reserved94: [u8; 0x08],
    ier_rn: IerRn,
    ierh_rn: IerhRn,
    iecr_rn: IecrRn,
    iecrh_rn: IecrhRn,
    iesr_rn: IesrRn,
    iesrh_rn: IesrhRn,
    ipr_rn: IprRn,
    iprh_rn: IprhRn,
    icr_rn: IcrRn,
    icrh_rn: IcrhRn,
    ieval_rn: IevalRn,
    _reserved105: [u8; 0x04],
    qer_rn: QerRn,
    qeer_rn: QeerRn,
    qeecr_rn: QeecrRn,
    qeesr_rn: QeesrRn,
    qser_rn: QserRn,
    qsecr_rn: QsecrRn,
    _reserved111: [u8; 0x1f68],
    opt: Opt,
    src: Src,
    abcnt: Abcnt,
    dst: Dst,
    bidx: Bidx,
    lnk: Lnk,
    cidx: Cidx,
    ccnt: Ccnt,
}
impl RegisterBlock {
    #[doc = "0x00 - Peripheral ID Register"]
    #[inline(always)]
    pub const fn pid(&self) -> &Pid {
        &self.pid
    }
    #[doc = "0x04 - CC Configuration Register"]
    #[inline(always)]
    pub const fn cccfg(&self) -> &Cccfg {
        &self.cccfg
    }
    #[doc = "0x200 - QDMA Channel N Mapping Register"]
    #[inline(always)]
    pub const fn qchmapn(&self) -> &Qchmapn {
        &self.qchmapn
    }
    #[doc = "0x240 - DMA Queue Number Register n Contains the Event queue number to be used for the corresponding DMA Channel."]
    #[inline(always)]
    pub const fn dmaqnumn(&self) -> &Dmaqnumn {
        &self.dmaqnumn
    }
    #[doc = "0x260 - QDMA Queue Number Register Contains the Event queue number to be used for the corresponding QDMA Channel."]
    #[inline(always)]
    pub const fn qdmaqnum(&self) -> &Qdmaqnum {
        &self.qdmaqnum
    }
    #[doc = "0x280 - Queue to TC Mapping"]
    #[inline(always)]
    pub const fn quetcmap(&self) -> &Quetcmap {
        &self.quetcmap
    }
    #[doc = "0x284 - Queue Priority"]
    #[inline(always)]
    pub const fn quepri(&self) -> &Quepri {
        &self.quepri
    }
    #[doc = "0x300 - Event Missed Register: The Event Missed register is set if 2 events are received without the first event being cleared or if a Null TR is serviced. Chained events (CER) Set Events (ESR) and normal events (ER) are treated individually. If any bit in the EMR register is set (and all errors (including QEMR/CCERR) were previously clear) then an error will be signaled with TPCC error interrupt."]
    #[inline(always)]
    pub const fn emr(&self) -> &Emr {
        &self.emr
    }
    #[doc = "0x304 - Event Missed Register (High Part): The Event Missed register is set if 2 events are received without the first event being cleared or if a Null TR is serviced. Chained events (CER) Set Events (ESR) and normal events (ER) are treated individually. If any bit in the EMR register is set (and all errors (including QEMR/CCERR) were previously clear) then an error will be signaled with TPCC error interrupt."]
    #[inline(always)]
    pub const fn emrh(&self) -> &Emrh {
        &self.emrh
    }
    #[doc = "0x308 - Event Missed Clear Register: CPU write of '1' to the EMCR.En bit causes the EMR.En bit to be cleared. CPU write of '0' has no effect.. All error bits must be cleared before additional error interrupts will be asserted by CC."]
    #[inline(always)]
    pub const fn emcr(&self) -> &Emcr {
        &self.emcr
    }
    #[doc = "0x30c - Event Missed Clear Register (High Part): CPU write of '1' to the EMCR.En bit causes the EMR.En bit to be cleared. CPU write of '0' has no effect.. All error bits must be cleared before additional error interrupts will be asserted by CC."]
    #[inline(always)]
    pub const fn emcrh(&self) -> &Emcrh {
        &self.emcrh
    }
    #[doc = "0x310 - QDMA Event Missed Register: The QDMA Event Missed register is set if 2 QDMA events are detected without the first event being cleared or if a Null TR is serviced.. If any bit in the QEMR register is set (and all errors (including EMR/CCERR) were previously clear) then an error will be signaled with TPCC error interrupt."]
    #[inline(always)]
    pub const fn qemr(&self) -> &Qemr {
        &self.qemr
    }
    #[doc = "0x314 - QDMA Event Missed Clear Register: CPU write of '1' to the QEMCR.En bit causes the QEMR.En bit to be cleared. CPU write of '0' has no effect.. All error bits must be cleared before additional error interrupts will be asserted by CC."]
    #[inline(always)]
    pub const fn qemcr(&self) -> &Qemcr {
        &self.qemcr
    }
    #[doc = "0x318 - CC Error Register"]
    #[inline(always)]
    pub const fn ccerr(&self) -> &Ccerr {
        &self.ccerr
    }
    #[doc = "0x31c - CC Error Clear Register"]
    #[inline(always)]
    pub const fn ccerrclr(&self) -> &Ccerrclr {
        &self.ccerrclr
    }
    #[doc = "0x320 - Error Eval Register"]
    #[inline(always)]
    pub const fn eeval(&self) -> &Eeval {
        &self.eeval
    }
    #[doc = "0x340 - DMA Region Access enable for bit N in Region M: En = 0 : Accesses via Region M address space to Bit N in any DMA Channel Register are not allowed. Reads will return 'b0 on Bit N and writes will not modify the state of bit N. Enabled interrupt bits for bit N do not contribute to the generation of the TPCC region M interrupt. En = 1 : Accesses via Region M address space to Bit N in any DMA Channel Register are allowed. Reads will return the value from Bit N and writes will modify the state of bit N. Enabled interrupt bits for bit N do contribute to the generation of the TPCC region M interrupt."]
    #[inline(always)]
    pub const fn draem(&self) -> &Draem {
        &self.draem
    }
    #[doc = "0x344 - DMA Region Access enable for bit N in Region M: En = 0 : Accesses via Region M address space to Bit N in any DMA Channel Register are not allowed. Reads will return 'b0 on Bit N and writes will not modify the state of bit N. Enabled interrupt bits for bit N do not contribute to the generation of the TPCC region M interrupt. En = 1 : Accesses via Region M address space to Bit N in any DMA Channel Register are allowed. Reads will return the value from Bit N and writes will modify the state of bit N. Enabled interrupt bits for bit N do contribute to the generation of the TPCC region M interrupt. En = 0 : Accesses via Region M address space to Bit N in any DMA Channel Register are not allowed. Reads will return 'b0 on Bit N and writes will not modify the state of bit N. Enabled interrupt bits for bit N do not contribute to the generation of the TPCC region M interrupt. En = 1 : Accesses via Region M address space to Bit N in any DMA Channel Register are allowed. Reads will return the value from Bit N and writes will modify the state of bit N. Enabled interrupt bits for bit N do contribute to the generation of the TPCC region M interrupt."]
    #[inline(always)]
    pub const fn draehm(&self) -> &Draehm {
        &self.draehm
    }
    #[doc = "0x380 - QDMA Region Access enable for bit N in Region M: En = 0 : Accesses via Region M address space to Bit N in any QDMA Channel Register are not allowed. Reads will return 'b0 on Bit N and writes will not modify the state of bit N. Enabled interrupt bits for bit N do not contribute to the generation of the TPCC region M interrupt. En = 1 : Accesses via Region M address space to Bit N in any QDMA Channel Register are allowed. Reads will return the value from Bit N and writes will modify the state of bit N. Enabled interrupt bits for bit N do contribute to the generation of the TPCC region n interrupt."]
    #[inline(always)]
    pub const fn qraen(&self) -> &Qraen {
        &self.qraen
    }
    #[doc = "0x400 - Event Queue Entry Diagram for Queue n - Entry 0"]
    #[inline(always)]
    pub const fn qne0(&self) -> &Qne0 {
        &self.qne0
    }
    #[doc = "0x404 - Event Queue Entry Diagram for Queue n - Entry 1"]
    #[inline(always)]
    pub const fn qne1(&self) -> &Qne1 {
        &self.qne1
    }
    #[doc = "0x408 - Event Queue Entry Diagram for Queue n - Entry 2"]
    #[inline(always)]
    pub const fn qne2(&self) -> &Qne2 {
        &self.qne2
    }
    #[doc = "0x40c - Event Queue Entry Diagram for Queue n - Entry 3"]
    #[inline(always)]
    pub const fn qne3(&self) -> &Qne3 {
        &self.qne3
    }
    #[doc = "0x410 - Event Queue Entry Diagram for Queue n - Entry 4"]
    #[inline(always)]
    pub const fn qne4(&self) -> &Qne4 {
        &self.qne4
    }
    #[doc = "0x414 - Event Queue Entry Diagram for Queue n - Entry 5"]
    #[inline(always)]
    pub const fn qne5(&self) -> &Qne5 {
        &self.qne5
    }
    #[doc = "0x418 - Event Queue Entry Diagram for Queue n - Entry 6"]
    #[inline(always)]
    pub const fn qne6(&self) -> &Qne6 {
        &self.qne6
    }
    #[doc = "0x41c - Event Queue Entry Diagram for Queue n - Entry 7"]
    #[inline(always)]
    pub const fn qne7(&self) -> &Qne7 {
        &self.qne7
    }
    #[doc = "0x420 - Event Queue Entry Diagram for Queue n - Entry 8"]
    #[inline(always)]
    pub const fn qne8(&self) -> &Qne8 {
        &self.qne8
    }
    #[doc = "0x424 - Event Queue Entry Diagram for Queue n - Entry 9"]
    #[inline(always)]
    pub const fn qne9(&self) -> &Qne9 {
        &self.qne9
    }
    #[doc = "0x428 - Event Queue Entry Diagram for Queue n - Entry 0"]
    #[inline(always)]
    pub const fn qne10(&self) -> &Qne10 {
        &self.qne10
    }
    #[doc = "0x42c - Event Queue Entry Diagram for Queue n - Entry 11"]
    #[inline(always)]
    pub const fn qne11(&self) -> &Qne11 {
        &self.qne11
    }
    #[doc = "0x430 - Event Queue Entry Diagram for Queue n - Entry 12"]
    #[inline(always)]
    pub const fn qne12(&self) -> &Qne12 {
        &self.qne12
    }
    #[doc = "0x434 - Event Queue Entry Diagram for Queue n - Entry 13"]
    #[inline(always)]
    pub const fn qne13(&self) -> &Qne13 {
        &self.qne13
    }
    #[doc = "0x438 - Event Queue Entry Diagram for Queue n - Entry 14"]
    #[inline(always)]
    pub const fn qne14(&self) -> &Qne14 {
        &self.qne14
    }
    #[doc = "0x43c - Event Queue Entry Diagram for Queue n - Entry 15"]
    #[inline(always)]
    pub const fn qne15(&self) -> &Qne15 {
        &self.qne15
    }
    #[doc = "0x600 - QSTATn Register Set"]
    #[inline(always)]
    pub const fn qstatn(&self) -> &Qstatn {
        &self.qstatn
    }
    #[doc = "0x620 - Queue Threshold A for Q\\[3:0\\]: CCERR.QTHRXCDn and QSTATn.THRXCD error bit is set when the number of Events in QueueN at an instant in time (visible via QSTATn.NUMVAL) equals or exceeds the value specified by QWMTHRA.Qn. Legal values = 0x0 (ever used?) to 0x10 (ever full?) A value of 0x11 disables threshold errors."]
    #[inline(always)]
    pub const fn qwmthra(&self) -> &Qwmthra {
        &self.qwmthra
    }
    #[doc = "0x640 - CC Status Register"]
    #[inline(always)]
    pub const fn ccstat(&self) -> &Ccstat {
        &self.ccstat
    }
    #[doc = "0x700 - Advanced Event Trigger Control"]
    #[inline(always)]
    pub const fn aetctl(&self) -> &Aetctl {
        &self.aetctl
    }
    #[doc = "0x704 - Advanced Event Trigger Stat"]
    #[inline(always)]
    pub const fn aetstat(&self) -> &Aetstat {
        &self.aetstat
    }
    #[doc = "0x708 - AET Command"]
    #[inline(always)]
    pub const fn aetcmd(&self) -> &Aetcmd {
        &self.aetcmd
    }
    #[doc = "0x1000 - Event Register: If ER.En bit is set and the EER.En bit is also set then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. ER.En bit is set when the input event #n transitions from inactive (low) to active (high) regardless of the state of EER.En bit. ER.En bit is cleared when the corresponding event is prioritized and serviced. If the ER.En bit is already set and a new inactive to active transition is detected on the input event #n input AND the corresponding bit in the EER register is set then the corresponding bit in the Event Missed Register is set. Event N can be cleared via sw by writing a '1' to the ECR pseudo-register."]
    #[inline(always)]
    pub const fn er(&self) -> &Er {
        &self.er
    }
    #[doc = "0x1004 - Event Register (High Part): If ERH.En bit is set and the EERH.En bit is also set then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. ERH.En bit is set when the input event #n transitions from inactive (low) to active (high) regardless of the state of EERH.En bit. ER.En bit is cleared when the corresponding event is prioritized and serviced. If the ERH.En bit is already set and a new inactive to active transition is detected on the input event #n input AND the corresponding bit in the EERH register is set then the corresponding bit in the Event Missed Register is set. Event N can be cleared via sw by writing a '1' to the ECRH pseudo-register."]
    #[inline(always)]
    pub const fn erh(&self) -> &Erh {
        &self.erh
    }
    #[doc = "0x1008 - Event Clear Register: CPU write of '1' to the ECR.En bit causes the ER.En bit to be cleared. CPU write of '0' has no effect."]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
    #[doc = "0x100c - Event Clear Register (High Part): CPU write of '1' to the ECRH.En bit causes the ERH.En bit to be cleared. CPU write of '0' has no effect."]
    #[inline(always)]
    pub const fn ecrh(&self) -> &Ecrh {
        &self.ecrh
    }
    #[doc = "0x1010 - Event Set Register: CPU write of '1' to the ESR.En bit causes the ER.En bit to be set. CPU write of '0' has no effect."]
    #[inline(always)]
    pub const fn esr(&self) -> &Esr {
        &self.esr
    }
    #[doc = "0x1014 - Event Set Register (High Part) CPU write of '1' to the ESRH.En bit causes the ERH.En bit to be set. CPU write of '0' has no effect."]
    #[inline(always)]
    pub const fn esrh(&self) -> &Esrh {
        &self.esrh
    }
    #[doc = "0x1018 - Chained Event Register: If CER.En bit is set (regardless of state of EER.En) then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. CER.En bit is set when a chaining completion code is returned from one of the 3PTCs via the completion interface or is generated internally via Early Completion path. CER.En bit is cleared when the corresponding event is prioritized and serviced. If the CER.En bit is already set and the corresponding chaining completion code is returned from the TC then the corresponding bit in the Event Missed Register is set. CER.En cannot be set or cleared via software."]
    #[inline(always)]
    pub const fn cer(&self) -> &Cer {
        &self.cer
    }
    #[doc = "0x101c - Chained Event Register (High Part): If CERH.En bit is set (regardless of state of EERH.En) then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. CERH.En bit is set when a chaining completion code is returned from one of the 3PTCs via the completion interface or is generated internally via Early Completion path. CERH.En bit is cleared when the corresponding event is prioritized and serviced. If the CERH.En bit is already set and the corresponding chaining completion code is returned from the TC then the corresponding bit in the Event Missed Register is set. CERH.En cannot be set or cleared via software."]
    #[inline(always)]
    pub const fn cerh(&self) -> &Cerh {
        &self.cerh
    }
    #[doc = "0x1020 - Event Enable Register: Enables DMA transfers for ER.En pending events. ER.En is set based on externally asserted events (via tpcc_eventN_pi). This register has no effect on Chained Event Register (CER) or Event Set Register (ESR). Note that if a bit is set in ER.En while EER.En is disabled no action is taken. If EER.En is enabled at a later point (and ER.En has not been cleared via SW) then the event will be recognized as a valid 'TR Sync' EER.En is not directly writeable. Events can be enabled via writes to EESR and can be disabled via writes to EECR register. EER.En = 0: ER.En is not enabled to trigger DMA transfers. EER.En = 1: ER.En is enabled to trigger DMA transfers."]
    #[inline(always)]
    pub const fn eer(&self) -> &Eer {
        &self.eer
    }
    #[doc = "0x1024 - Event Enable Register (High Part): Enables DMA transfers for ERH.En pending events. ERH.En is set based on externally asserted events (via tpcc_eventN_pi). This register has no effect on Chained Event Register (CERH) or Event Set Register (ESRH). Note that if a bit is set in ERH.En while EERH.En is disabled no action is taken. If EERH.En is enabled at a later point (and ERH.En has not been cleared via SW) then the event will be recognized as a valid 'TR Sync' EERH.En is not directly writeable. Events can be enabled via writes to EESRH and can be disabled via writes to EECRH register. EERH.En = 0: ER.En is not enabled to trigger DMA transfers. EERH.En = 1: ER.En is enabled to trigger DMA transfers."]
    #[inline(always)]
    pub const fn eerh(&self) -> &Eerh {
        &self.eerh
    }
    #[doc = "0x1028 - Event Enable Clear Register: CPU write of '1' to the EECR.En bit causes the EER.En bit to be cleared. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn eecr(&self) -> &Eecr {
        &self.eecr
    }
    #[doc = "0x102c - Event Enable Clear Register (High Part): CPU write of '1' to the EECRH.En bit causes the EERH.En bit to be cleared. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn eecrh(&self) -> &Eecrh {
        &self.eecrh
    }
    #[doc = "0x1030 - Event Enable Set Register: CPU write of '1' to the EESR.En bit causes the EER.En bit to be set. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn eesr(&self) -> &Eesr {
        &self.eesr
    }
    #[doc = "0x1034 - Event Enable Set Register (High Part): CPU write of '1' to the EESRH.En bit causes the EERH.En bit to be set. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn eesrh(&self) -> &Eesrh {
        &self.eesrh
    }
    #[doc = "0x1038 - Secondary Event Register: The secondary event register is used along with the Event Register (ER) to provide information on the state of an Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events."]
    #[inline(always)]
    pub const fn ser(&self) -> &Ser {
        &self.ser
    }
    #[doc = "0x103c - Secondary Event Register (High Part): The secondary event register is used along with the Event Register (ERH) to provide information on the state of an Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events."]
    #[inline(always)]
    pub const fn serh(&self) -> &Serh {
        &self.serh
    }
    #[doc = "0x1040 - Secondary Event Clear Register: The secondary event clear register is used to clear the status of the SER registers. CPU write of '1' to the SECR.En bit clears the SER register. CPU write of '0' has no effect."]
    #[inline(always)]
    pub const fn secr(&self) -> &Secr {
        &self.secr
    }
    #[doc = "0x1044 - Secondary Event Clear Register (High Part): The secondary event clear register is used to clear the status of the SERH registers. CPU write of '1' to the SECRH.En bit clears the SERH register. CPU write of '0' has no effect."]
    #[inline(always)]
    pub const fn secrh(&self) -> &Secrh {
        &self.secrh
    }
    #[doc = "0x1050 - Int Enable Register: IER.In is not directly writeable. Interrupts can be enabled via writes to IESR and can be disabled via writes to IECR register. IER.In = 0: IPR.In is NOT enabled for interrupts. IER.In = 1: IPR.In IS enabled for interrupts."]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x1054 - Int Enable Register (High Part): IERH.In is not directly writeable. Interrupts can be enabled via writes to IESRH and can be disabled via writes to IECRH register. IERH.In = 0: IPRH.In is NOT enabled for interrupts. IERH.In = 1: IPRH.In IS enabled for interrupts."]
    #[inline(always)]
    pub const fn ierh(&self) -> &Ierh {
        &self.ierh
    }
    #[doc = "0x1058 - Int Enable Clear Register: CPU write of '1' to the IECR.In bit causes the IER.In bit to be cleared. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn iecr(&self) -> &Iecr {
        &self.iecr
    }
    #[doc = "0x105c - Int Enable Clear Register (High Part): CPU write of '1' to the IECRH.In bit causes the IERH.In bit to be cleared. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn iecrh(&self) -> &Iecrh {
        &self.iecrh
    }
    #[doc = "0x1060 - Int Enable Set Register: CPU write of '1' to the IESR.In bit causes the IESR.In bit to be set. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn iesr(&self) -> &Iesr {
        &self.iesr
    }
    #[doc = "0x1064 - Int Enable Set Register (High Part): CPU write of '1' to the IESRH.In bit causes the IESRH.In bit to be set. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn iesrh(&self) -> &Iesrh {
        &self.iesrh
    }
    #[doc = "0x1068 - Interrupt Pending Register: IPR.In bit is set when a interrupt completion code with TCC of N is detected. IPR.In bit is cleared via software by writing a '1' to ICR.In bit."]
    #[inline(always)]
    pub const fn ipr(&self) -> &Ipr {
        &self.ipr
    }
    #[doc = "0x106c - Interrupt Pending Register (High Part): IPRH.In bit is set when a interrupt completion code with TCC of N is detected. IPRH.In bit is cleared via software by writing a '1' to ICRH.In bit."]
    #[inline(always)]
    pub const fn iprh(&self) -> &Iprh {
        &self.iprh
    }
    #[doc = "0x1070 - Interrupt Clear Register: CPU write of '1' to the ICR.In bit causes the IPR.In bit to be cleared. CPU write of '0' has no effect. All IPR.In bits must be cleared before additional interrupts will be asserted by CC."]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x1074 - Interrupt Clear Register (High Part): CPU write of '1' to the ICRH.In bit causes the IPRH.In bit to be cleared. CPU write of '0' has no effect. All IPRH.In bits must be cleared before additional interrupts will be asserted by CC."]
    #[inline(always)]
    pub const fn icrh(&self) -> &Icrh {
        &self.icrh
    }
    #[doc = "0x1078 - Interrupt Eval Register"]
    #[inline(always)]
    pub const fn ieval(&self) -> &Ieval {
        &self.ieval
    }
    #[doc = "0x1080 - QDMA Event Register: If QER.En bit is set then the corresponding QDMA channel is prioritized vs. other qdma events for submission to the TC. QER.En bit is set when a vbus write byte matches the address defined in the QCHMAPn register. QER.En bit is cleared when the corresponding event is prioritized and serviced. QER.En is also cleared when user writes a '1' to the QSECR.En bit. If the QER.En bit is already set and a new QDMA event is detected due to user write to QDMA trigger location and QEER register is set then the corresponding bit in the QDMA Event Missed Register is set."]
    #[inline(always)]
    pub const fn qer(&self) -> &Qer {
        &self.qer
    }
    #[doc = "0x1084 - QDMA Event Enable Register: Enabled/disabled QDMA address comparator for QDMA Channel N. QEER.En is not directly writeable. QDMA channels can be enabled via writes to QEESR and can be disabled via writes to QEECR register. QEER.En = 1 The corresponding QDMA channel comparator is enabled and Events will be recognized and latched in QER.En. QEER.En = 0 The corresponding QDMA channel comparator is disabled. Events will not be recognized/latched in QER.En."]
    #[inline(always)]
    pub const fn qeer(&self) -> &Qeer {
        &self.qeer
    }
    #[doc = "0x1088 - QDMA Event Enable Clear Register: CPU write of '1' to the QEECR.En bit causes the QEER.En bit to be cleared. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn qeecr(&self) -> &Qeecr {
        &self.qeecr
    }
    #[doc = "0x108c - QDMA Event Enable Set Register: CPU write of '1' to the QEESR.En bit causes the QEESR.En bit to be set. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn qeesr(&self) -> &Qeesr {
        &self.qeesr
    }
    #[doc = "0x1090 - QDMA Secondary Event Register: The QDMA secondary event register is used along with the QDMA Event Register (QER) to provide information on the state of a QDMA Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events."]
    #[inline(always)]
    pub const fn qser(&self) -> &Qser {
        &self.qser
    }
    #[doc = "0x1094 - QDMA Secondary Event Clear Register: The secondary event clear register is used to clear the status of the QSER and QER register (note that this is slightly different than the SER operation which does not clear the ER.En register). CPU write of '1' to the QSECR.En bit clears the QSER.En and QER.En register fields. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn qsecr(&self) -> &Qsecr {
        &self.qsecr
    }
    #[doc = "0x2000 - Event Register: If ER.En bit is set and the EER.En bit is also set then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. ER.En bit is set when the input event #n transitions from inactive (low) to active (high) regardless of the state of EER.En bit. ER.En bit is cleared when the corresponding event is prioritized and serviced. If the ER.En bit is already set and a new inactive to active transition is detected on the input event #n input AND the corresponding bit in the EER register is set then the corresponding bit in the Event Missed Register is set. Event N can be cleared via sw by writing a '1' to the ECR pseudo-register."]
    #[inline(always)]
    pub const fn er_rn(&self) -> &ErRn {
        &self.er_rn
    }
    #[doc = "0x2004 - Event Register (High Part): If ERH.En bit is set and the EERH.En bit is also set then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. ERH.En bit is set when the input event #n transitions from inactive (low) to active (high) regardless of the state of EERH.En bit. ER.En bit is cleared when the corresponding event is prioritized and serviced. If the ERH.En bit is already set and a new inactive to active transition is detected on the input event #n input AND the corresponding bit in the EERH register is set then the corresponding bit in the Event Missed Register is set. Event N can be cleared via sw by writing a '1' to the ECRH pseudo-register."]
    #[inline(always)]
    pub const fn erh_rn(&self) -> &ErhRn {
        &self.erh_rn
    }
    #[doc = "0x2008 - Event Clear Register: CPU write of '1' to the ECR.En bit causes the ER.En bit to be cleared. CPU write of '0' has no effect."]
    #[inline(always)]
    pub const fn ecr_rn(&self) -> &EcrRn {
        &self.ecr_rn
    }
    #[doc = "0x200c - Event Clear Register (High Part): CPU write of '1' to the ECRH.En bit causes the ERH.En bit to be cleared. CPU write of '0' has no effect."]
    #[inline(always)]
    pub const fn ecrh_rn(&self) -> &EcrhRn {
        &self.ecrh_rn
    }
    #[doc = "0x2010 - Event Set Register: CPU write of '1' to the ESR.En bit causes the ER.En bit to be set. CPU write of '0' has no effect."]
    #[inline(always)]
    pub const fn esr_rn(&self) -> &EsrRn {
        &self.esr_rn
    }
    #[doc = "0x2014 - Event Set Register (High Part) CPU write of '1' to the ESRH.En bit causes the ERH.En bit to be set. CPU write of '0' has no effect."]
    #[inline(always)]
    pub const fn esrh_rn(&self) -> &EsrhRn {
        &self.esrh_rn
    }
    #[doc = "0x2018 - Chained Event Register: If CER.En bit is set (regardless of state of EER.En) then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. CER.En bit is set when a chaining completion code is returned from one of the 3PTCs via the completion interface or is generated internally via Early Completion path. CER.En bit is cleared when the corresponding event is prioritized and serviced. If the CER.En bit is already set and the corresponding chaining completion code is returned from the TC then the corresponding bit in the Event Missed Register is set. CER.En cannot be set or cleared via software."]
    #[inline(always)]
    pub const fn cer_rn(&self) -> &CerRn {
        &self.cer_rn
    }
    #[doc = "0x201c - Chained Event Register (High Part): If CERH.En bit is set (regardless of state of EERH.En) then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. CERH.En bit is set when a chaining completion code is returned from one of the 3PTCs via the completion interface or is generated internally via Early Completion path. CERH.En bit is cleared when the corresponding event is prioritized and serviced. If the CERH.En bit is already set and the corresponding chaining completion code is returned from the TC then the corresponding bit in the Event Missed Register is set. CERH.En cannot be set or cleared via software."]
    #[inline(always)]
    pub const fn cerh_rn(&self) -> &CerhRn {
        &self.cerh_rn
    }
    #[doc = "0x2020 - Event Enable Register: Enables DMA transfers for ER.En pending events. ER.En is set based on externally asserted events (via tpcc_eventN_pi). This register has no effect on Chained Event Register (CER) or Event Set Register (ESR). Note that if a bit is set in ER.En while EER.En is disabled no action is taken. If EER.En is enabled at a later point (and ER.En has not been cleared via SW) then the event will be recognized as a valid 'TR Sync' EER.En is not directly writeable. Events can be enabled via writes to EESR and can be disabled via writes to EECR register. EER.En = 0: ER.En is not enabled to trigger DMA transfers. EER.En = 1: ER.En is enabled to trigger DMA transfers."]
    #[inline(always)]
    pub const fn eer_rn(&self) -> &EerRn {
        &self.eer_rn
    }
    #[doc = "0x2024 - Event Enable Register (High Part): Enables DMA transfers for ERH.En pending events. ERH.En is set based on externally asserted events (via tpcc_eventN_pi). This register has no effect on Chained Event Register (CERH) or Event Set Register (ESRH). Note that if a bit is set in ERH.En while EERH.En is disabled no action is taken. If EERH.En is enabled at a later point (and ERH.En has not been cleared via SW) then the event will be recognized as a valid 'TR Sync' EERH.En is not directly writeable. Events can be enabled via writes to EESRH and can be disabled via writes to EECRH register. EERH.En = 0: ER.En is not enabled to trigger DMA transfers. EERH.En = 1: ER.En is enabled to trigger DMA transfers."]
    #[inline(always)]
    pub const fn eerh_rn(&self) -> &EerhRn {
        &self.eerh_rn
    }
    #[doc = "0x2028 - Event Enable Clear Register: CPU write of '1' to the EECR.En bit causes the EER.En bit to be cleared. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn eecr_rn(&self) -> &EecrRn {
        &self.eecr_rn
    }
    #[doc = "0x202c - Event Enable Clear Register (High Part): CPU write of '1' to the EECRH.En bit causes the EERH.En bit to be cleared. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn eecrh_rn(&self) -> &EecrhRn {
        &self.eecrh_rn
    }
    #[doc = "0x2030 - Event Enable Set Register: CPU write of '1' to the EESR.En bit causes the EER.En bit to be set. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn eesr_rn(&self) -> &EesrRn {
        &self.eesr_rn
    }
    #[doc = "0x2034 - Event Enable Set Register (High Part): CPU write of '1' to the EESRH.En bit causes the EERH.En bit to be set. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn eesrh_rn(&self) -> &EesrhRn {
        &self.eesrh_rn
    }
    #[doc = "0x2038 - Secondary Event Register: The secondary event register is used along with the Event Register (ER) to provide information on the state of an Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events."]
    #[inline(always)]
    pub const fn ser_rn(&self) -> &SerRn {
        &self.ser_rn
    }
    #[doc = "0x203c - Secondary Event Register (High Part): The secondary event register is used along with the Event Register (ERH) to provide information on the state of an Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events."]
    #[inline(always)]
    pub const fn serh_rn(&self) -> &SerhRn {
        &self.serh_rn
    }
    #[doc = "0x2040 - Secondary Event Clear Register: The secondary event clear register is used to clear the status of the SER registers. CPU write of '1' to the SECR.En bit clears the SER register. CPU write of '0' has no effect."]
    #[inline(always)]
    pub const fn secr_rn(&self) -> &SecrRn {
        &self.secr_rn
    }
    #[doc = "0x2044 - Secondary Event Clear Register (High Part): The secondary event clear register is used to clear the status of the SERH registers. CPU write of '1' to the SECRH.En bit clears the SERH register. CPU write of '0' has no effect."]
    #[inline(always)]
    pub const fn secrh_rn(&self) -> &SecrhRn {
        &self.secrh_rn
    }
    #[doc = "0x2050 - Int Enable Register: IER.In is not directly writeable. Interrupts can be enabled via writes to IESR and can be disabled via writes to IECR register. IER.In = 0: IPR.In is NOT enabled for interrupts. IER.In = 1: IPR.In IS enabled for interrupts."]
    #[inline(always)]
    pub const fn ier_rn(&self) -> &IerRn {
        &self.ier_rn
    }
    #[doc = "0x2054 - Int Enable Register (High Part): IERH.In is not directly writeable. Interrupts can be enabled via writes to IESRH and can be disabled via writes to IECRH register. IERH.In = 0: IPRH.In is NOT enabled for interrupts. IERH.In = 1: IPRH.In IS enabled for interrupts."]
    #[inline(always)]
    pub const fn ierh_rn(&self) -> &IerhRn {
        &self.ierh_rn
    }
    #[doc = "0x2058 - Int Enable Clear Register: CPU write of '1' to the IECR.In bit causes the IER.In bit to be cleared. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn iecr_rn(&self) -> &IecrRn {
        &self.iecr_rn
    }
    #[doc = "0x205c - Int Enable Clear Register (High Part): CPU write of '1' to the IECRH.In bit causes the IERH.In bit to be cleared. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn iecrh_rn(&self) -> &IecrhRn {
        &self.iecrh_rn
    }
    #[doc = "0x2060 - Int Enable Set Register: CPU write of '1' to the IESR.In bit causes the IESR.In bit to be set. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn iesr_rn(&self) -> &IesrRn {
        &self.iesr_rn
    }
    #[doc = "0x2064 - Int Enable Set Register (High Part): CPU write of '1' to the IESRH.In bit causes the IESRH.In bit to be set. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn iesrh_rn(&self) -> &IesrhRn {
        &self.iesrh_rn
    }
    #[doc = "0x2068 - Interrupt Pending Register: IPR.In bit is set when a interrupt completion code with TCC of N is detected. IPR.In bit is cleared via software by writing a '1' to ICR.In bit."]
    #[inline(always)]
    pub const fn ipr_rn(&self) -> &IprRn {
        &self.ipr_rn
    }
    #[doc = "0x206c - Interrupt Pending Register (High Part): IPRH.In bit is set when a interrupt completion code with TCC of N is detected. IPRH.In bit is cleared via software by writing a '1' to ICRH.In bit."]
    #[inline(always)]
    pub const fn iprh_rn(&self) -> &IprhRn {
        &self.iprh_rn
    }
    #[doc = "0x2070 - Interrupt Clear Register: CPU write of '1' to the ICR.In bit causes the IPR.In bit to be cleared. CPU write of '0' has no effect. All IPR.In bits must be cleared before additional interrupts will be asserted by CC."]
    #[inline(always)]
    pub const fn icr_rn(&self) -> &IcrRn {
        &self.icr_rn
    }
    #[doc = "0x2074 - Interrupt Clear Register (High Part): CPU write of '1' to the ICRH.In bit causes the IPRH.In bit to be cleared. CPU write of '0' has no effect. All IPRH.In bits must be cleared before additional interrupts will be asserted by CC."]
    #[inline(always)]
    pub const fn icrh_rn(&self) -> &IcrhRn {
        &self.icrh_rn
    }
    #[doc = "0x2078 - Interrupt Eval Register"]
    #[inline(always)]
    pub const fn ieval_rn(&self) -> &IevalRn {
        &self.ieval_rn
    }
    #[doc = "0x2080 - QDMA Event Register: If QER.En bit is set then the corresponding QDMA channel is prioritized vs. other qdma events for submission to the TC. QER.En bit is set when a vbus write byte matches the address defined in the QCHMAPn register. QER.En bit is cleared when the corresponding event is prioritized and serviced. QER.En is also cleared when user writes a '1' to the QSECR.En bit. If the QER.En bit is already set and a new QDMA event is detected due to user write to QDMA trigger location and QEER register is set then the corresponding bit in the QDMA Event Missed Register is set."]
    #[inline(always)]
    pub const fn qer_rn(&self) -> &QerRn {
        &self.qer_rn
    }
    #[doc = "0x2084 - QDMA Event Enable Register: Enabled/disabled QDMA address comparator for QDMA Channel N. QEER.En is not directly writeable. QDMA channels can be enabled via writes to QEESR and can be disabled via writes to QEECR register. QEER.En = 1 The corresponding QDMA channel comparator is enabled and Events will be recognized and latched in QER.En. QEER.En = 0 The corresponding QDMA channel comparator is disabled. Events will not be recognized/latched in QER.En."]
    #[inline(always)]
    pub const fn qeer_rn(&self) -> &QeerRn {
        &self.qeer_rn
    }
    #[doc = "0x2088 - QDMA Event Enable Clear Register: CPU write of '1' to the QEECR.En bit causes the QEER.En bit to be cleared. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn qeecr_rn(&self) -> &QeecrRn {
        &self.qeecr_rn
    }
    #[doc = "0x208c - QDMA Event Enable Set Register: CPU write of '1' to the QEESR.En bit causes the QEESR.En bit to be set. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn qeesr_rn(&self) -> &QeesrRn {
        &self.qeesr_rn
    }
    #[doc = "0x2090 - QDMA Secondary Event Register: The QDMA secondary event register is used along with the QDMA Event Register (QER) to provide information on the state of a QDMA Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events."]
    #[inline(always)]
    pub const fn qser_rn(&self) -> &QserRn {
        &self.qser_rn
    }
    #[doc = "0x2094 - QDMA Secondary Event Clear Register: The secondary event clear register is used to clear the status of the QSER and QER register (note that this is slightly different than the SER operation which does not clear the ER.En register). CPU write of '1' to the QSECR.En bit clears the QSER.En and QER.En register fields. CPU write of '0' has no effect.."]
    #[inline(always)]
    pub const fn qsecr_rn(&self) -> &QsecrRn {
        &self.qsecr_rn
    }
    #[doc = "0x4000 - Options Parameter"]
    #[inline(always)]
    pub const fn opt(&self) -> &Opt {
        &self.opt
    }
    #[doc = "0x4004 - Source Address"]
    #[inline(always)]
    pub const fn src(&self) -> &Src {
        &self.src
    }
    #[doc = "0x4008 - A and B byte count"]
    #[inline(always)]
    pub const fn abcnt(&self) -> &Abcnt {
        &self.abcnt
    }
    #[doc = "0x400c - Destination Address"]
    #[inline(always)]
    pub const fn dst(&self) -> &Dst {
        &self.dst
    }
    #[doc = "0x4010 - Register description is not available"]
    #[inline(always)]
    pub const fn bidx(&self) -> &Bidx {
        &self.bidx
    }
    #[doc = "0x4014 - Link and Reload parameters"]
    #[inline(always)]
    pub const fn lnk(&self) -> &Lnk {
        &self.lnk
    }
    #[doc = "0x4018 - Register description is not available"]
    #[inline(always)]
    pub const fn cidx(&self) -> &Cidx {
        &self.cidx
    }
    #[doc = "0x401c - C byte count"]
    #[inline(always)]
    pub const fn ccnt(&self) -> &Ccnt {
        &self.ccnt
    }
}
#[doc = "PID (rw) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
#[doc(alias = "PID")]
pub type Pid = crate::Reg<pid::PidSpec>;
#[doc = "Peripheral ID Register"]
pub mod pid;
#[doc = "CCCFG (rw) register accessor: CC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccfg`]
module"]
#[doc(alias = "CCCFG")]
pub type Cccfg = crate::Reg<cccfg::CccfgSpec>;
#[doc = "CC Configuration Register"]
pub mod cccfg;
#[doc = "QCHMAPN (rw) register accessor: QDMA Channel N Mapping Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qchmapn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qchmapn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qchmapn`]
module"]
#[doc(alias = "QCHMAPN")]
pub type Qchmapn = crate::Reg<qchmapn::QchmapnSpec>;
#[doc = "QDMA Channel N Mapping Register"]
pub mod qchmapn;
#[doc = "DMAQNUMN (rw) register accessor: DMA Queue Number Register n Contains the Event queue number to be used for the corresponding DMA Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaqnumn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaqnumn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaqnumn`]
module"]
#[doc(alias = "DMAQNUMN")]
pub type Dmaqnumn = crate::Reg<dmaqnumn::DmaqnumnSpec>;
#[doc = "DMA Queue Number Register n Contains the Event queue number to be used for the corresponding DMA Channel."]
pub mod dmaqnumn;
#[doc = "QDMAQNUM (rw) register accessor: QDMA Queue Number Register Contains the Event queue number to be used for the corresponding QDMA Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`qdmaqnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdmaqnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qdmaqnum`]
module"]
#[doc(alias = "QDMAQNUM")]
pub type Qdmaqnum = crate::Reg<qdmaqnum::QdmaqnumSpec>;
#[doc = "QDMA Queue Number Register Contains the Event queue number to be used for the corresponding QDMA Channel."]
pub mod qdmaqnum;
#[doc = "QUETCMAP (rw) register accessor: Queue to TC Mapping\n\nYou can [`read`](crate::Reg::read) this register and get [`quetcmap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quetcmap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quetcmap`]
module"]
#[doc(alias = "QUETCMAP")]
pub type Quetcmap = crate::Reg<quetcmap::QuetcmapSpec>;
#[doc = "Queue to TC Mapping"]
pub mod quetcmap;
#[doc = "QUEPRI (rw) register accessor: Queue Priority\n\nYou can [`read`](crate::Reg::read) this register and get [`quepri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quepri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quepri`]
module"]
#[doc(alias = "QUEPRI")]
pub type Quepri = crate::Reg<quepri::QuepriSpec>;
#[doc = "Queue Priority"]
pub mod quepri;
#[doc = "EMR (rw) register accessor: Event Missed Register: The Event Missed register is set if 2 events are received without the first event being cleared or if a Null TR is serviced. Chained events (CER) Set Events (ESR) and normal events (ER) are treated individually. If any bit in the EMR register is set (and all errors (including QEMR/CCERR) were previously clear) then an error will be signaled with TPCC error interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr`]
module"]
#[doc(alias = "EMR")]
pub type Emr = crate::Reg<emr::EmrSpec>;
#[doc = "Event Missed Register: The Event Missed register is set if 2 events are received without the first event being cleared or if a Null TR is serviced. Chained events (CER) Set Events (ESR) and normal events (ER) are treated individually. If any bit in the EMR register is set (and all errors (including QEMR/CCERR) were previously clear) then an error will be signaled with TPCC error interrupt."]
pub mod emr;
#[doc = "EMRH (rw) register accessor: Event Missed Register (High Part): The Event Missed register is set if 2 events are received without the first event being cleared or if a Null TR is serviced. Chained events (CER) Set Events (ESR) and normal events (ER) are treated individually. If any bit in the EMR register is set (and all errors (including QEMR/CCERR) were previously clear) then an error will be signaled with TPCC error interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`emrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emrh`]
module"]
#[doc(alias = "EMRH")]
pub type Emrh = crate::Reg<emrh::EmrhSpec>;
#[doc = "Event Missed Register (High Part): The Event Missed register is set if 2 events are received without the first event being cleared or if a Null TR is serviced. Chained events (CER) Set Events (ESR) and normal events (ER) are treated individually. If any bit in the EMR register is set (and all errors (including QEMR/CCERR) were previously clear) then an error will be signaled with TPCC error interrupt."]
pub mod emrh;
#[doc = "EMCR (rw) register accessor: Event Missed Clear Register: CPU write of '1' to the EMCR.En bit causes the EMR.En bit to be cleared. CPU write of '0' has no effect.. All error bits must be cleared before additional error interrupts will be asserted by CC.\n\nYou can [`read`](crate::Reg::read) this register and get [`emcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emcr`]
module"]
#[doc(alias = "EMCR")]
pub type Emcr = crate::Reg<emcr::EmcrSpec>;
#[doc = "Event Missed Clear Register: CPU write of '1' to the EMCR.En bit causes the EMR.En bit to be cleared. CPU write of '0' has no effect.. All error bits must be cleared before additional error interrupts will be asserted by CC."]
pub mod emcr;
#[doc = "EMCRH (rw) register accessor: Event Missed Clear Register (High Part): CPU write of '1' to the EMCR.En bit causes the EMR.En bit to be cleared. CPU write of '0' has no effect.. All error bits must be cleared before additional error interrupts will be asserted by CC.\n\nYou can [`read`](crate::Reg::read) this register and get [`emcrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emcrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emcrh`]
module"]
#[doc(alias = "EMCRH")]
pub type Emcrh = crate::Reg<emcrh::EmcrhSpec>;
#[doc = "Event Missed Clear Register (High Part): CPU write of '1' to the EMCR.En bit causes the EMR.En bit to be cleared. CPU write of '0' has no effect.. All error bits must be cleared before additional error interrupts will be asserted by CC."]
pub mod emcrh;
#[doc = "QEMR (rw) register accessor: QDMA Event Missed Register: The QDMA Event Missed register is set if 2 QDMA events are detected without the first event being cleared or if a Null TR is serviced.. If any bit in the QEMR register is set (and all errors (including EMR/CCERR) were previously clear) then an error will be signaled with TPCC error interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`qemr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qemr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qemr`]
module"]
#[doc(alias = "QEMR")]
pub type Qemr = crate::Reg<qemr::QemrSpec>;
#[doc = "QDMA Event Missed Register: The QDMA Event Missed register is set if 2 QDMA events are detected without the first event being cleared or if a Null TR is serviced.. If any bit in the QEMR register is set (and all errors (including EMR/CCERR) were previously clear) then an error will be signaled with TPCC error interrupt."]
pub mod qemr;
#[doc = "QEMCR (rw) register accessor: QDMA Event Missed Clear Register: CPU write of '1' to the QEMCR.En bit causes the QEMR.En bit to be cleared. CPU write of '0' has no effect.. All error bits must be cleared before additional error interrupts will be asserted by CC.\n\nYou can [`read`](crate::Reg::read) this register and get [`qemcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qemcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qemcr`]
module"]
#[doc(alias = "QEMCR")]
pub type Qemcr = crate::Reg<qemcr::QemcrSpec>;
#[doc = "QDMA Event Missed Clear Register: CPU write of '1' to the QEMCR.En bit causes the QEMR.En bit to be cleared. CPU write of '0' has no effect.. All error bits must be cleared before additional error interrupts will be asserted by CC."]
pub mod qemcr;
#[doc = "CCERR (rw) register accessor: CC Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccerr`]
module"]
#[doc(alias = "CCERR")]
pub type Ccerr = crate::Reg<ccerr::CcerrSpec>;
#[doc = "CC Error Register"]
pub mod ccerr;
#[doc = "CCERRCLR (rw) register accessor: CC Error Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccerrclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccerrclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccerrclr`]
module"]
#[doc(alias = "CCERRCLR")]
pub type Ccerrclr = crate::Reg<ccerrclr::CcerrclrSpec>;
#[doc = "CC Error Clear Register"]
pub mod ccerrclr;
#[doc = "EEVAL (rw) register accessor: Error Eval Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eeval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eeval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeval`]
module"]
#[doc(alias = "EEVAL")]
pub type Eeval = crate::Reg<eeval::EevalSpec>;
#[doc = "Error Eval Register"]
pub mod eeval;
#[doc = "DRAEM (rw) register accessor: DMA Region Access enable for bit N in Region M: En = 0 : Accesses via Region M address space to Bit N in any DMA Channel Register are not allowed. Reads will return 'b0 on Bit N and writes will not modify the state of bit N. Enabled interrupt bits for bit N do not contribute to the generation of the TPCC region M interrupt. En = 1 : Accesses via Region M address space to Bit N in any DMA Channel Register are allowed. Reads will return the value from Bit N and writes will modify the state of bit N. Enabled interrupt bits for bit N do contribute to the generation of the TPCC region M interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`draem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`draem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@draem`]
module"]
#[doc(alias = "DRAEM")]
pub type Draem = crate::Reg<draem::DraemSpec>;
#[doc = "DMA Region Access enable for bit N in Region M: En = 0 : Accesses via Region M address space to Bit N in any DMA Channel Register are not allowed. Reads will return 'b0 on Bit N and writes will not modify the state of bit N. Enabled interrupt bits for bit N do not contribute to the generation of the TPCC region M interrupt. En = 1 : Accesses via Region M address space to Bit N in any DMA Channel Register are allowed. Reads will return the value from Bit N and writes will modify the state of bit N. Enabled interrupt bits for bit N do contribute to the generation of the TPCC region M interrupt."]
pub mod draem;
#[doc = "DRAEHM (rw) register accessor: DMA Region Access enable for bit N in Region M: En = 0 : Accesses via Region M address space to Bit N in any DMA Channel Register are not allowed. Reads will return 'b0 on Bit N and writes will not modify the state of bit N. Enabled interrupt bits for bit N do not contribute to the generation of the TPCC region M interrupt. En = 1 : Accesses via Region M address space to Bit N in any DMA Channel Register are allowed. Reads will return the value from Bit N and writes will modify the state of bit N. Enabled interrupt bits for bit N do contribute to the generation of the TPCC region M interrupt. En = 0 : Accesses via Region M address space to Bit N in any DMA Channel Register are not allowed. Reads will return 'b0 on Bit N and writes will not modify the state of bit N. Enabled interrupt bits for bit N do not contribute to the generation of the TPCC region M interrupt. En = 1 : Accesses via Region M address space to Bit N in any DMA Channel Register are allowed. Reads will return the value from Bit N and writes will modify the state of bit N. Enabled interrupt bits for bit N do contribute to the generation of the TPCC region M interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`draehm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`draehm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@draehm`]
module"]
#[doc(alias = "DRAEHM")]
pub type Draehm = crate::Reg<draehm::DraehmSpec>;
#[doc = "DMA Region Access enable for bit N in Region M: En = 0 : Accesses via Region M address space to Bit N in any DMA Channel Register are not allowed. Reads will return 'b0 on Bit N and writes will not modify the state of bit N. Enabled interrupt bits for bit N do not contribute to the generation of the TPCC region M interrupt. En = 1 : Accesses via Region M address space to Bit N in any DMA Channel Register are allowed. Reads will return the value from Bit N and writes will modify the state of bit N. Enabled interrupt bits for bit N do contribute to the generation of the TPCC region M interrupt. En = 0 : Accesses via Region M address space to Bit N in any DMA Channel Register are not allowed. Reads will return 'b0 on Bit N and writes will not modify the state of bit N. Enabled interrupt bits for bit N do not contribute to the generation of the TPCC region M interrupt. En = 1 : Accesses via Region M address space to Bit N in any DMA Channel Register are allowed. Reads will return the value from Bit N and writes will modify the state of bit N. Enabled interrupt bits for bit N do contribute to the generation of the TPCC region M interrupt."]
pub mod draehm;
#[doc = "QRAEN (rw) register accessor: QDMA Region Access enable for bit N in Region M: En = 0 : Accesses via Region M address space to Bit N in any QDMA Channel Register are not allowed. Reads will return 'b0 on Bit N and writes will not modify the state of bit N. Enabled interrupt bits for bit N do not contribute to the generation of the TPCC region M interrupt. En = 1 : Accesses via Region M address space to Bit N in any QDMA Channel Register are allowed. Reads will return the value from Bit N and writes will modify the state of bit N. Enabled interrupt bits for bit N do contribute to the generation of the TPCC region n interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`qraen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qraen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qraen`]
module"]
#[doc(alias = "QRAEN")]
pub type Qraen = crate::Reg<qraen::QraenSpec>;
#[doc = "QDMA Region Access enable for bit N in Region M: En = 0 : Accesses via Region M address space to Bit N in any QDMA Channel Register are not allowed. Reads will return 'b0 on Bit N and writes will not modify the state of bit N. Enabled interrupt bits for bit N do not contribute to the generation of the TPCC region M interrupt. En = 1 : Accesses via Region M address space to Bit N in any QDMA Channel Register are allowed. Reads will return the value from Bit N and writes will modify the state of bit N. Enabled interrupt bits for bit N do contribute to the generation of the TPCC region n interrupt."]
pub mod qraen;
#[doc = "QNE0 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 0\n\nYou can [`read`](crate::Reg::read) this register and get [`qne0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne0`]
module"]
#[doc(alias = "QNE0")]
pub type Qne0 = crate::Reg<qne0::Qne0Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 0"]
pub mod qne0;
#[doc = "QNE1 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 1\n\nYou can [`read`](crate::Reg::read) this register and get [`qne1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne1`]
module"]
#[doc(alias = "QNE1")]
pub type Qne1 = crate::Reg<qne1::Qne1Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 1"]
pub mod qne1;
#[doc = "QNE2 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 2\n\nYou can [`read`](crate::Reg::read) this register and get [`qne2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne2`]
module"]
#[doc(alias = "QNE2")]
pub type Qne2 = crate::Reg<qne2::Qne2Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 2"]
pub mod qne2;
#[doc = "QNE3 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 3\n\nYou can [`read`](crate::Reg::read) this register and get [`qne3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne3`]
module"]
#[doc(alias = "QNE3")]
pub type Qne3 = crate::Reg<qne3::Qne3Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 3"]
pub mod qne3;
#[doc = "QNE4 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 4\n\nYou can [`read`](crate::Reg::read) this register and get [`qne4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne4`]
module"]
#[doc(alias = "QNE4")]
pub type Qne4 = crate::Reg<qne4::Qne4Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 4"]
pub mod qne4;
#[doc = "QNE5 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 5\n\nYou can [`read`](crate::Reg::read) this register and get [`qne5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne5`]
module"]
#[doc(alias = "QNE5")]
pub type Qne5 = crate::Reg<qne5::Qne5Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 5"]
pub mod qne5;
#[doc = "QNE6 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 6\n\nYou can [`read`](crate::Reg::read) this register and get [`qne6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne6`]
module"]
#[doc(alias = "QNE6")]
pub type Qne6 = crate::Reg<qne6::Qne6Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 6"]
pub mod qne6;
#[doc = "QNE7 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 7\n\nYou can [`read`](crate::Reg::read) this register and get [`qne7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne7`]
module"]
#[doc(alias = "QNE7")]
pub type Qne7 = crate::Reg<qne7::Qne7Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 7"]
pub mod qne7;
#[doc = "QNE8 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 8\n\nYou can [`read`](crate::Reg::read) this register and get [`qne8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne8`]
module"]
#[doc(alias = "QNE8")]
pub type Qne8 = crate::Reg<qne8::Qne8Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 8"]
pub mod qne8;
#[doc = "QNE9 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 9\n\nYou can [`read`](crate::Reg::read) this register and get [`qne9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne9`]
module"]
#[doc(alias = "QNE9")]
pub type Qne9 = crate::Reg<qne9::Qne9Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 9"]
pub mod qne9;
#[doc = "QNE10 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 0\n\nYou can [`read`](crate::Reg::read) this register and get [`qne10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne10`]
module"]
#[doc(alias = "QNE10")]
pub type Qne10 = crate::Reg<qne10::Qne10Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 0"]
pub mod qne10;
#[doc = "QNE11 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 11\n\nYou can [`read`](crate::Reg::read) this register and get [`qne11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne11`]
module"]
#[doc(alias = "QNE11")]
pub type Qne11 = crate::Reg<qne11::Qne11Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 11"]
pub mod qne11;
#[doc = "QNE12 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 12\n\nYou can [`read`](crate::Reg::read) this register and get [`qne12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne12`]
module"]
#[doc(alias = "QNE12")]
pub type Qne12 = crate::Reg<qne12::Qne12Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 12"]
pub mod qne12;
#[doc = "QNE13 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 13\n\nYou can [`read`](crate::Reg::read) this register and get [`qne13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne13`]
module"]
#[doc(alias = "QNE13")]
pub type Qne13 = crate::Reg<qne13::Qne13Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 13"]
pub mod qne13;
#[doc = "QNE14 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 14\n\nYou can [`read`](crate::Reg::read) this register and get [`qne14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne14`]
module"]
#[doc(alias = "QNE14")]
pub type Qne14 = crate::Reg<qne14::Qne14Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 14"]
pub mod qne14;
#[doc = "QNE15 (rw) register accessor: Event Queue Entry Diagram for Queue n - Entry 15\n\nYou can [`read`](crate::Reg::read) this register and get [`qne15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qne15`]
module"]
#[doc(alias = "QNE15")]
pub type Qne15 = crate::Reg<qne15::Qne15Spec>;
#[doc = "Event Queue Entry Diagram for Queue n - Entry 15"]
pub mod qne15;
#[doc = "QSTATN (rw) register accessor: QSTATn Register Set\n\nYou can [`read`](crate::Reg::read) this register and get [`qstatn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qstatn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qstatn`]
module"]
#[doc(alias = "QSTATN")]
pub type Qstatn = crate::Reg<qstatn::QstatnSpec>;
#[doc = "QSTATn Register Set"]
pub mod qstatn;
#[doc = "QWMTHRA (rw) register accessor: Queue Threshold A for Q\\[3:0\\]: CCERR.QTHRXCDn and QSTATn.THRXCD error bit is set when the number of Events in QueueN at an instant in time (visible via QSTATn.NUMVAL) equals or exceeds the value specified by QWMTHRA.Qn. Legal values = 0x0 (ever used?) to 0x10 (ever full?) A value of 0x11 disables threshold errors.\n\nYou can [`read`](crate::Reg::read) this register and get [`qwmthra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qwmthra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qwmthra`]
module"]
#[doc(alias = "QWMTHRA")]
pub type Qwmthra = crate::Reg<qwmthra::QwmthraSpec>;
#[doc = "Queue Threshold A for Q\\[3:0\\]: CCERR.QTHRXCDn and QSTATn.THRXCD error bit is set when the number of Events in QueueN at an instant in time (visible via QSTATn.NUMVAL) equals or exceeds the value specified by QWMTHRA.Qn. Legal values = 0x0 (ever used?) to 0x10 (ever full?) A value of 0x11 disables threshold errors."]
pub mod qwmthra;
#[doc = "CCSTAT (rw) register accessor: CC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccstat`]
module"]
#[doc(alias = "CCSTAT")]
pub type Ccstat = crate::Reg<ccstat::CcstatSpec>;
#[doc = "CC Status Register"]
pub mod ccstat;
#[doc = "AETCTL (rw) register accessor: Advanced Event Trigger Control\n\nYou can [`read`](crate::Reg::read) this register and get [`aetctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aetctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aetctl`]
module"]
#[doc(alias = "AETCTL")]
pub type Aetctl = crate::Reg<aetctl::AetctlSpec>;
#[doc = "Advanced Event Trigger Control"]
pub mod aetctl;
#[doc = "AETSTAT (rw) register accessor: Advanced Event Trigger Stat\n\nYou can [`read`](crate::Reg::read) this register and get [`aetstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aetstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aetstat`]
module"]
#[doc(alias = "AETSTAT")]
pub type Aetstat = crate::Reg<aetstat::AetstatSpec>;
#[doc = "Advanced Event Trigger Stat"]
pub mod aetstat;
#[doc = "AETCMD (rw) register accessor: AET Command\n\nYou can [`read`](crate::Reg::read) this register and get [`aetcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aetcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aetcmd`]
module"]
#[doc(alias = "AETCMD")]
pub type Aetcmd = crate::Reg<aetcmd::AetcmdSpec>;
#[doc = "AET Command"]
pub mod aetcmd;
#[doc = "ER (rw) register accessor: Event Register: If ER.En bit is set and the EER.En bit is also set then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. ER.En bit is set when the input event #n transitions from inactive (low) to active (high) regardless of the state of EER.En bit. ER.En bit is cleared when the corresponding event is prioritized and serviced. If the ER.En bit is already set and a new inactive to active transition is detected on the input event #n input AND the corresponding bit in the EER register is set then the corresponding bit in the Event Missed Register is set. Event N can be cleared via sw by writing a '1' to the ECR pseudo-register.\n\nYou can [`read`](crate::Reg::read) this register and get [`er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@er`]
module"]
#[doc(alias = "ER")]
pub type Er = crate::Reg<er::ErSpec>;
#[doc = "Event Register: If ER.En bit is set and the EER.En bit is also set then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. ER.En bit is set when the input event #n transitions from inactive (low) to active (high) regardless of the state of EER.En bit. ER.En bit is cleared when the corresponding event is prioritized and serviced. If the ER.En bit is already set and a new inactive to active transition is detected on the input event #n input AND the corresponding bit in the EER register is set then the corresponding bit in the Event Missed Register is set. Event N can be cleared via sw by writing a '1' to the ECR pseudo-register."]
pub mod er;
#[doc = "ERH (rw) register accessor: Event Register (High Part): If ERH.En bit is set and the EERH.En bit is also set then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. ERH.En bit is set when the input event #n transitions from inactive (low) to active (high) regardless of the state of EERH.En bit. ER.En bit is cleared when the corresponding event is prioritized and serviced. If the ERH.En bit is already set and a new inactive to active transition is detected on the input event #n input AND the corresponding bit in the EERH register is set then the corresponding bit in the Event Missed Register is set. Event N can be cleared via sw by writing a '1' to the ECRH pseudo-register.\n\nYou can [`read`](crate::Reg::read) this register and get [`erh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erh`]
module"]
#[doc(alias = "ERH")]
pub type Erh = crate::Reg<erh::ErhSpec>;
#[doc = "Event Register (High Part): If ERH.En bit is set and the EERH.En bit is also set then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. ERH.En bit is set when the input event #n transitions from inactive (low) to active (high) regardless of the state of EERH.En bit. ER.En bit is cleared when the corresponding event is prioritized and serviced. If the ERH.En bit is already set and a new inactive to active transition is detected on the input event #n input AND the corresponding bit in the EERH register is set then the corresponding bit in the Event Missed Register is set. Event N can be cleared via sw by writing a '1' to the ECRH pseudo-register."]
pub mod erh;
#[doc = "ECR (rw) register accessor: Event Clear Register: CPU write of '1' to the ECR.En bit causes the ER.En bit to be cleared. CPU write of '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "Event Clear Register: CPU write of '1' to the ECR.En bit causes the ER.En bit to be cleared. CPU write of '0' has no effect."]
pub mod ecr;
#[doc = "ECRH (rw) register accessor: Event Clear Register (High Part): CPU write of '1' to the ECRH.En bit causes the ERH.En bit to be cleared. CPU write of '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecrh`]
module"]
#[doc(alias = "ECRH")]
pub type Ecrh = crate::Reg<ecrh::EcrhSpec>;
#[doc = "Event Clear Register (High Part): CPU write of '1' to the ECRH.En bit causes the ERH.En bit to be cleared. CPU write of '0' has no effect."]
pub mod ecrh;
#[doc = "ESR (rw) register accessor: Event Set Register: CPU write of '1' to the ESR.En bit causes the ER.En bit to be set. CPU write of '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`esr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esr`]
module"]
#[doc(alias = "ESR")]
pub type Esr = crate::Reg<esr::EsrSpec>;
#[doc = "Event Set Register: CPU write of '1' to the ESR.En bit causes the ER.En bit to be set. CPU write of '0' has no effect."]
pub mod esr;
#[doc = "ESRH (rw) register accessor: Event Set Register (High Part) CPU write of '1' to the ESRH.En bit causes the ERH.En bit to be set. CPU write of '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`esrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esrh`]
module"]
#[doc(alias = "ESRH")]
pub type Esrh = crate::Reg<esrh::EsrhSpec>;
#[doc = "Event Set Register (High Part) CPU write of '1' to the ESRH.En bit causes the ERH.En bit to be set. CPU write of '0' has no effect."]
pub mod esrh;
#[doc = "CER (rw) register accessor: Chained Event Register: If CER.En bit is set (regardless of state of EER.En) then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. CER.En bit is set when a chaining completion code is returned from one of the 3PTCs via the completion interface or is generated internally via Early Completion path. CER.En bit is cleared when the corresponding event is prioritized and serviced. If the CER.En bit is already set and the corresponding chaining completion code is returned from the TC then the corresponding bit in the Event Missed Register is set. CER.En cannot be set or cleared via software.\n\nYou can [`read`](crate::Reg::read) this register and get [`cer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cer`]
module"]
#[doc(alias = "CER")]
pub type Cer = crate::Reg<cer::CerSpec>;
#[doc = "Chained Event Register: If CER.En bit is set (regardless of state of EER.En) then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. CER.En bit is set when a chaining completion code is returned from one of the 3PTCs via the completion interface or is generated internally via Early Completion path. CER.En bit is cleared when the corresponding event is prioritized and serviced. If the CER.En bit is already set and the corresponding chaining completion code is returned from the TC then the corresponding bit in the Event Missed Register is set. CER.En cannot be set or cleared via software."]
pub mod cer;
#[doc = "CERH (rw) register accessor: Chained Event Register (High Part): If CERH.En bit is set (regardless of state of EERH.En) then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. CERH.En bit is set when a chaining completion code is returned from one of the 3PTCs via the completion interface or is generated internally via Early Completion path. CERH.En bit is cleared when the corresponding event is prioritized and serviced. If the CERH.En bit is already set and the corresponding chaining completion code is returned from the TC then the corresponding bit in the Event Missed Register is set. CERH.En cannot be set or cleared via software.\n\nYou can [`read`](crate::Reg::read) this register and get [`cerh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cerh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cerh`]
module"]
#[doc(alias = "CERH")]
pub type Cerh = crate::Reg<cerh::CerhSpec>;
#[doc = "Chained Event Register (High Part): If CERH.En bit is set (regardless of state of EERH.En) then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. CERH.En bit is set when a chaining completion code is returned from one of the 3PTCs via the completion interface or is generated internally via Early Completion path. CERH.En bit is cleared when the corresponding event is prioritized and serviced. If the CERH.En bit is already set and the corresponding chaining completion code is returned from the TC then the corresponding bit in the Event Missed Register is set. CERH.En cannot be set or cleared via software."]
pub mod cerh;
#[doc = "EER (rw) register accessor: Event Enable Register: Enables DMA transfers for ER.En pending events. ER.En is set based on externally asserted events (via tpcc_eventN_pi). This register has no effect on Chained Event Register (CER) or Event Set Register (ESR). Note that if a bit is set in ER.En while EER.En is disabled no action is taken. If EER.En is enabled at a later point (and ER.En has not been cleared via SW) then the event will be recognized as a valid 'TR Sync' EER.En is not directly writeable. Events can be enabled via writes to EESR and can be disabled via writes to EECR register. EER.En = 0: ER.En is not enabled to trigger DMA transfers. EER.En = 1: ER.En is enabled to trigger DMA transfers.\n\nYou can [`read`](crate::Reg::read) this register and get [`eer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eer`]
module"]
#[doc(alias = "EER")]
pub type Eer = crate::Reg<eer::EerSpec>;
#[doc = "Event Enable Register: Enables DMA transfers for ER.En pending events. ER.En is set based on externally asserted events (via tpcc_eventN_pi). This register has no effect on Chained Event Register (CER) or Event Set Register (ESR). Note that if a bit is set in ER.En while EER.En is disabled no action is taken. If EER.En is enabled at a later point (and ER.En has not been cleared via SW) then the event will be recognized as a valid 'TR Sync' EER.En is not directly writeable. Events can be enabled via writes to EESR and can be disabled via writes to EECR register. EER.En = 0: ER.En is not enabled to trigger DMA transfers. EER.En = 1: ER.En is enabled to trigger DMA transfers."]
pub mod eer;
#[doc = "EERH (rw) register accessor: Event Enable Register (High Part): Enables DMA transfers for ERH.En pending events. ERH.En is set based on externally asserted events (via tpcc_eventN_pi). This register has no effect on Chained Event Register (CERH) or Event Set Register (ESRH). Note that if a bit is set in ERH.En while EERH.En is disabled no action is taken. If EERH.En is enabled at a later point (and ERH.En has not been cleared via SW) then the event will be recognized as a valid 'TR Sync' EERH.En is not directly writeable. Events can be enabled via writes to EESRH and can be disabled via writes to EECRH register. EERH.En = 0: ER.En is not enabled to trigger DMA transfers. EERH.En = 1: ER.En is enabled to trigger DMA transfers.\n\nYou can [`read`](crate::Reg::read) this register and get [`eerh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eerh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eerh`]
module"]
#[doc(alias = "EERH")]
pub type Eerh = crate::Reg<eerh::EerhSpec>;
#[doc = "Event Enable Register (High Part): Enables DMA transfers for ERH.En pending events. ERH.En is set based on externally asserted events (via tpcc_eventN_pi). This register has no effect on Chained Event Register (CERH) or Event Set Register (ESRH). Note that if a bit is set in ERH.En while EERH.En is disabled no action is taken. If EERH.En is enabled at a later point (and ERH.En has not been cleared via SW) then the event will be recognized as a valid 'TR Sync' EERH.En is not directly writeable. Events can be enabled via writes to EESRH and can be disabled via writes to EECRH register. EERH.En = 0: ER.En is not enabled to trigger DMA transfers. EERH.En = 1: ER.En is enabled to trigger DMA transfers."]
pub mod eerh;
#[doc = "EECR (rw) register accessor: Event Enable Clear Register: CPU write of '1' to the EECR.En bit causes the EER.En bit to be cleared. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`eecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecr`]
module"]
#[doc(alias = "EECR")]
pub type Eecr = crate::Reg<eecr::EecrSpec>;
#[doc = "Event Enable Clear Register: CPU write of '1' to the EECR.En bit causes the EER.En bit to be cleared. CPU write of '0' has no effect.."]
pub mod eecr;
#[doc = "EECRH (rw) register accessor: Event Enable Clear Register (High Part): CPU write of '1' to the EECRH.En bit causes the EERH.En bit to be cleared. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`eecrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecrh`]
module"]
#[doc(alias = "EECRH")]
pub type Eecrh = crate::Reg<eecrh::EecrhSpec>;
#[doc = "Event Enable Clear Register (High Part): CPU write of '1' to the EECRH.En bit causes the EERH.En bit to be cleared. CPU write of '0' has no effect.."]
pub mod eecrh;
#[doc = "EESR (rw) register accessor: Event Enable Set Register: CPU write of '1' to the EESR.En bit causes the EER.En bit to be set. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`eesr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eesr`]
module"]
#[doc(alias = "EESR")]
pub type Eesr = crate::Reg<eesr::EesrSpec>;
#[doc = "Event Enable Set Register: CPU write of '1' to the EESR.En bit causes the EER.En bit to be set. CPU write of '0' has no effect.."]
pub mod eesr;
#[doc = "EESRH (rw) register accessor: Event Enable Set Register (High Part): CPU write of '1' to the EESRH.En bit causes the EERH.En bit to be set. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`eesrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eesrh`]
module"]
#[doc(alias = "EESRH")]
pub type Eesrh = crate::Reg<eesrh::EesrhSpec>;
#[doc = "Event Enable Set Register (High Part): CPU write of '1' to the EESRH.En bit causes the EERH.En bit to be set. CPU write of '0' has no effect.."]
pub mod eesrh;
#[doc = "SER (rw) register accessor: Secondary Event Register: The secondary event register is used along with the Event Register (ER) to provide information on the state of an Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events.\n\nYou can [`read`](crate::Reg::read) this register and get [`ser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ser`]
module"]
#[doc(alias = "SER")]
pub type Ser = crate::Reg<ser::SerSpec>;
#[doc = "Secondary Event Register: The secondary event register is used along with the Event Register (ER) to provide information on the state of an Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events."]
pub mod ser;
#[doc = "SERH (rw) register accessor: Secondary Event Register (High Part): The secondary event register is used along with the Event Register (ERH) to provide information on the state of an Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events.\n\nYou can [`read`](crate::Reg::read) this register and get [`serh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`serh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@serh`]
module"]
#[doc(alias = "SERH")]
pub type Serh = crate::Reg<serh::SerhSpec>;
#[doc = "Secondary Event Register (High Part): The secondary event register is used along with the Event Register (ERH) to provide information on the state of an Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events."]
pub mod serh;
#[doc = "SECR (rw) register accessor: Secondary Event Clear Register: The secondary event clear register is used to clear the status of the SER registers. CPU write of '1' to the SECR.En bit clears the SER register. CPU write of '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`secr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secr`]
module"]
#[doc(alias = "SECR")]
pub type Secr = crate::Reg<secr::SecrSpec>;
#[doc = "Secondary Event Clear Register: The secondary event clear register is used to clear the status of the SER registers. CPU write of '1' to the SECR.En bit clears the SER register. CPU write of '0' has no effect."]
pub mod secr;
#[doc = "SECRH (rw) register accessor: Secondary Event Clear Register (High Part): The secondary event clear register is used to clear the status of the SERH registers. CPU write of '1' to the SECRH.En bit clears the SERH register. CPU write of '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`secrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secrh`]
module"]
#[doc(alias = "SECRH")]
pub type Secrh = crate::Reg<secrh::SecrhSpec>;
#[doc = "Secondary Event Clear Register (High Part): The secondary event clear register is used to clear the status of the SERH registers. CPU write of '1' to the SECRH.En bit clears the SERH register. CPU write of '0' has no effect."]
pub mod secrh;
#[doc = "IER (rw) register accessor: Int Enable Register: IER.In is not directly writeable. Interrupts can be enabled via writes to IESR and can be disabled via writes to IECR register. IER.In = 0: IPR.In is NOT enabled for interrupts. IER.In = 1: IPR.In IS enabled for interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Int Enable Register: IER.In is not directly writeable. Interrupts can be enabled via writes to IESR and can be disabled via writes to IECR register. IER.In = 0: IPR.In is NOT enabled for interrupts. IER.In = 1: IPR.In IS enabled for interrupts."]
pub mod ier;
#[doc = "IERH (rw) register accessor: Int Enable Register (High Part): IERH.In is not directly writeable. Interrupts can be enabled via writes to IESRH and can be disabled via writes to IECRH register. IERH.In = 0: IPRH.In is NOT enabled for interrupts. IERH.In = 1: IPRH.In IS enabled for interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`ierh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ierh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ierh`]
module"]
#[doc(alias = "IERH")]
pub type Ierh = crate::Reg<ierh::IerhSpec>;
#[doc = "Int Enable Register (High Part): IERH.In is not directly writeable. Interrupts can be enabled via writes to IESRH and can be disabled via writes to IECRH register. IERH.In = 0: IPRH.In is NOT enabled for interrupts. IERH.In = 1: IPRH.In IS enabled for interrupts."]
pub mod ierh;
#[doc = "IECR (rw) register accessor: Int Enable Clear Register: CPU write of '1' to the IECR.In bit causes the IER.In bit to be cleared. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`iecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iecr`]
module"]
#[doc(alias = "IECR")]
pub type Iecr = crate::Reg<iecr::IecrSpec>;
#[doc = "Int Enable Clear Register: CPU write of '1' to the IECR.In bit causes the IER.In bit to be cleared. CPU write of '0' has no effect.."]
pub mod iecr;
#[doc = "IECRH (rw) register accessor: Int Enable Clear Register (High Part): CPU write of '1' to the IECRH.In bit causes the IERH.In bit to be cleared. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`iecrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iecrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iecrh`]
module"]
#[doc(alias = "IECRH")]
pub type Iecrh = crate::Reg<iecrh::IecrhSpec>;
#[doc = "Int Enable Clear Register (High Part): CPU write of '1' to the IECRH.In bit causes the IERH.In bit to be cleared. CPU write of '0' has no effect.."]
pub mod iecrh;
#[doc = "IESR (rw) register accessor: Int Enable Set Register: CPU write of '1' to the IESR.In bit causes the IESR.In bit to be set. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`iesr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iesr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iesr`]
module"]
#[doc(alias = "IESR")]
pub type Iesr = crate::Reg<iesr::IesrSpec>;
#[doc = "Int Enable Set Register: CPU write of '1' to the IESR.In bit causes the IESR.In bit to be set. CPU write of '0' has no effect.."]
pub mod iesr;
#[doc = "IESRH (rw) register accessor: Int Enable Set Register (High Part): CPU write of '1' to the IESRH.In bit causes the IESRH.In bit to be set. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`iesrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iesrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iesrh`]
module"]
#[doc(alias = "IESRH")]
pub type Iesrh = crate::Reg<iesrh::IesrhSpec>;
#[doc = "Int Enable Set Register (High Part): CPU write of '1' to the IESRH.In bit causes the IESRH.In bit to be set. CPU write of '0' has no effect.."]
pub mod iesrh;
#[doc = "IPR (rw) register accessor: Interrupt Pending Register: IPR.In bit is set when a interrupt completion code with TCC of N is detected. IPR.In bit is cleared via software by writing a '1' to ICR.In bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr`]
module"]
#[doc(alias = "IPR")]
pub type Ipr = crate::Reg<ipr::IprSpec>;
#[doc = "Interrupt Pending Register: IPR.In bit is set when a interrupt completion code with TCC of N is detected. IPR.In bit is cleared via software by writing a '1' to ICR.In bit."]
pub mod ipr;
#[doc = "IPRH (rw) register accessor: Interrupt Pending Register (High Part): IPRH.In bit is set when a interrupt completion code with TCC of N is detected. IPRH.In bit is cleared via software by writing a '1' to ICRH.In bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`iprh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprh`]
module"]
#[doc(alias = "IPRH")]
pub type Iprh = crate::Reg<iprh::IprhSpec>;
#[doc = "Interrupt Pending Register (High Part): IPRH.In bit is set when a interrupt completion code with TCC of N is detected. IPRH.In bit is cleared via software by writing a '1' to ICRH.In bit."]
pub mod iprh;
#[doc = "ICR (rw) register accessor: Interrupt Clear Register: CPU write of '1' to the ICR.In bit causes the IPR.In bit to be cleared. CPU write of '0' has no effect. All IPR.In bits must be cleared before additional interrupts will be asserted by CC.\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt Clear Register: CPU write of '1' to the ICR.In bit causes the IPR.In bit to be cleared. CPU write of '0' has no effect. All IPR.In bits must be cleared before additional interrupts will be asserted by CC."]
pub mod icr;
#[doc = "ICRH (rw) register accessor: Interrupt Clear Register (High Part): CPU write of '1' to the ICRH.In bit causes the IPRH.In bit to be cleared. CPU write of '0' has no effect. All IPRH.In bits must be cleared before additional interrupts will be asserted by CC.\n\nYou can [`read`](crate::Reg::read) this register and get [`icrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icrh`]
module"]
#[doc(alias = "ICRH")]
pub type Icrh = crate::Reg<icrh::IcrhSpec>;
#[doc = "Interrupt Clear Register (High Part): CPU write of '1' to the ICRH.In bit causes the IPRH.In bit to be cleared. CPU write of '0' has no effect. All IPRH.In bits must be cleared before additional interrupts will be asserted by CC."]
pub mod icrh;
#[doc = "IEVAL (rw) register accessor: Interrupt Eval Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ieval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ieval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieval`]
module"]
#[doc(alias = "IEVAL")]
pub type Ieval = crate::Reg<ieval::IevalSpec>;
#[doc = "Interrupt Eval Register"]
pub mod ieval;
#[doc = "QER (rw) register accessor: QDMA Event Register: If QER.En bit is set then the corresponding QDMA channel is prioritized vs. other qdma events for submission to the TC. QER.En bit is set when a vbus write byte matches the address defined in the QCHMAPn register. QER.En bit is cleared when the corresponding event is prioritized and serviced. QER.En is also cleared when user writes a '1' to the QSECR.En bit. If the QER.En bit is already set and a new QDMA event is detected due to user write to QDMA trigger location and QEER register is set then the corresponding bit in the QDMA Event Missed Register is set.\n\nYou can [`read`](crate::Reg::read) this register and get [`qer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qer`]
module"]
#[doc(alias = "QER")]
pub type Qer = crate::Reg<qer::QerSpec>;
#[doc = "QDMA Event Register: If QER.En bit is set then the corresponding QDMA channel is prioritized vs. other qdma events for submission to the TC. QER.En bit is set when a vbus write byte matches the address defined in the QCHMAPn register. QER.En bit is cleared when the corresponding event is prioritized and serviced. QER.En is also cleared when user writes a '1' to the QSECR.En bit. If the QER.En bit is already set and a new QDMA event is detected due to user write to QDMA trigger location and QEER register is set then the corresponding bit in the QDMA Event Missed Register is set."]
pub mod qer;
#[doc = "QEER (rw) register accessor: QDMA Event Enable Register: Enabled/disabled QDMA address comparator for QDMA Channel N. QEER.En is not directly writeable. QDMA channels can be enabled via writes to QEESR and can be disabled via writes to QEECR register. QEER.En = 1 The corresponding QDMA channel comparator is enabled and Events will be recognized and latched in QER.En. QEER.En = 0 The corresponding QDMA channel comparator is disabled. Events will not be recognized/latched in QER.En.\n\nYou can [`read`](crate::Reg::read) this register and get [`qeer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qeer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qeer`]
module"]
#[doc(alias = "QEER")]
pub type Qeer = crate::Reg<qeer::QeerSpec>;
#[doc = "QDMA Event Enable Register: Enabled/disabled QDMA address comparator for QDMA Channel N. QEER.En is not directly writeable. QDMA channels can be enabled via writes to QEESR and can be disabled via writes to QEECR register. QEER.En = 1 The corresponding QDMA channel comparator is enabled and Events will be recognized and latched in QER.En. QEER.En = 0 The corresponding QDMA channel comparator is disabled. Events will not be recognized/latched in QER.En."]
pub mod qeer;
#[doc = "QEECR (rw) register accessor: QDMA Event Enable Clear Register: CPU write of '1' to the QEECR.En bit causes the QEER.En bit to be cleared. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`qeecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qeecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qeecr`]
module"]
#[doc(alias = "QEECR")]
pub type Qeecr = crate::Reg<qeecr::QeecrSpec>;
#[doc = "QDMA Event Enable Clear Register: CPU write of '1' to the QEECR.En bit causes the QEER.En bit to be cleared. CPU write of '0' has no effect.."]
pub mod qeecr;
#[doc = "QEESR (rw) register accessor: QDMA Event Enable Set Register: CPU write of '1' to the QEESR.En bit causes the QEESR.En bit to be set. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`qeesr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qeesr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qeesr`]
module"]
#[doc(alias = "QEESR")]
pub type Qeesr = crate::Reg<qeesr::QeesrSpec>;
#[doc = "QDMA Event Enable Set Register: CPU write of '1' to the QEESR.En bit causes the QEESR.En bit to be set. CPU write of '0' has no effect.."]
pub mod qeesr;
#[doc = "QSER (rw) register accessor: QDMA Secondary Event Register: The QDMA secondary event register is used along with the QDMA Event Register (QER) to provide information on the state of a QDMA Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events.\n\nYou can [`read`](crate::Reg::read) this register and get [`qser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qser`]
module"]
#[doc(alias = "QSER")]
pub type Qser = crate::Reg<qser::QserSpec>;
#[doc = "QDMA Secondary Event Register: The QDMA secondary event register is used along with the QDMA Event Register (QER) to provide information on the state of a QDMA Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events."]
pub mod qser;
#[doc = "QSECR (rw) register accessor: QDMA Secondary Event Clear Register: The secondary event clear register is used to clear the status of the QSER and QER register (note that this is slightly different than the SER operation which does not clear the ER.En register). CPU write of '1' to the QSECR.En bit clears the QSER.En and QER.En register fields. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`qsecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qsecr`]
module"]
#[doc(alias = "QSECR")]
pub type Qsecr = crate::Reg<qsecr::QsecrSpec>;
#[doc = "QDMA Secondary Event Clear Register: The secondary event clear register is used to clear the status of the QSER and QER register (note that this is slightly different than the SER operation which does not clear the ER.En register). CPU write of '1' to the QSECR.En bit clears the QSER.En and QER.En register fields. CPU write of '0' has no effect.."]
pub mod qsecr;
#[doc = "ER_RN (rw) register accessor: Event Register: If ER.En bit is set and the EER.En bit is also set then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. ER.En bit is set when the input event #n transitions from inactive (low) to active (high) regardless of the state of EER.En bit. ER.En bit is cleared when the corresponding event is prioritized and serviced. If the ER.En bit is already set and a new inactive to active transition is detected on the input event #n input AND the corresponding bit in the EER register is set then the corresponding bit in the Event Missed Register is set. Event N can be cleared via sw by writing a '1' to the ECR pseudo-register.\n\nYou can [`read`](crate::Reg::read) this register and get [`er_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`er_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@er_rn`]
module"]
#[doc(alias = "ER_RN")]
pub type ErRn = crate::Reg<er_rn::ErRnSpec>;
#[doc = "Event Register: If ER.En bit is set and the EER.En bit is also set then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. ER.En bit is set when the input event #n transitions from inactive (low) to active (high) regardless of the state of EER.En bit. ER.En bit is cleared when the corresponding event is prioritized and serviced. If the ER.En bit is already set and a new inactive to active transition is detected on the input event #n input AND the corresponding bit in the EER register is set then the corresponding bit in the Event Missed Register is set. Event N can be cleared via sw by writing a '1' to the ECR pseudo-register."]
pub mod er_rn;
#[doc = "ERH_RN (rw) register accessor: Event Register (High Part): If ERH.En bit is set and the EERH.En bit is also set then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. ERH.En bit is set when the input event #n transitions from inactive (low) to active (high) regardless of the state of EERH.En bit. ER.En bit is cleared when the corresponding event is prioritized and serviced. If the ERH.En bit is already set and a new inactive to active transition is detected on the input event #n input AND the corresponding bit in the EERH register is set then the corresponding bit in the Event Missed Register is set. Event N can be cleared via sw by writing a '1' to the ECRH pseudo-register.\n\nYou can [`read`](crate::Reg::read) this register and get [`erh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erh_rn`]
module"]
#[doc(alias = "ERH_RN")]
pub type ErhRn = crate::Reg<erh_rn::ErhRnSpec>;
#[doc = "Event Register (High Part): If ERH.En bit is set and the EERH.En bit is also set then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. ERH.En bit is set when the input event #n transitions from inactive (low) to active (high) regardless of the state of EERH.En bit. ER.En bit is cleared when the corresponding event is prioritized and serviced. If the ERH.En bit is already set and a new inactive to active transition is detected on the input event #n input AND the corresponding bit in the EERH register is set then the corresponding bit in the Event Missed Register is set. Event N can be cleared via sw by writing a '1' to the ECRH pseudo-register."]
pub mod erh_rn;
#[doc = "ECR_RN (rw) register accessor: Event Clear Register: CPU write of '1' to the ECR.En bit causes the ER.En bit to be cleared. CPU write of '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr_rn`]
module"]
#[doc(alias = "ECR_RN")]
pub type EcrRn = crate::Reg<ecr_rn::EcrRnSpec>;
#[doc = "Event Clear Register: CPU write of '1' to the ECR.En bit causes the ER.En bit to be cleared. CPU write of '0' has no effect."]
pub mod ecr_rn;
#[doc = "ECRH_RN (rw) register accessor: Event Clear Register (High Part): CPU write of '1' to the ECRH.En bit causes the ERH.En bit to be cleared. CPU write of '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecrh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecrh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecrh_rn`]
module"]
#[doc(alias = "ECRH_RN")]
pub type EcrhRn = crate::Reg<ecrh_rn::EcrhRnSpec>;
#[doc = "Event Clear Register (High Part): CPU write of '1' to the ECRH.En bit causes the ERH.En bit to be cleared. CPU write of '0' has no effect."]
pub mod ecrh_rn;
#[doc = "ESR_RN (rw) register accessor: Event Set Register: CPU write of '1' to the ESR.En bit causes the ER.En bit to be set. CPU write of '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`esr_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esr_rn`]
module"]
#[doc(alias = "ESR_RN")]
pub type EsrRn = crate::Reg<esr_rn::EsrRnSpec>;
#[doc = "Event Set Register: CPU write of '1' to the ESR.En bit causes the ER.En bit to be set. CPU write of '0' has no effect."]
pub mod esr_rn;
#[doc = "ESRH_RN (rw) register accessor: Event Set Register (High Part) CPU write of '1' to the ESRH.En bit causes the ERH.En bit to be set. CPU write of '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`esrh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esrh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esrh_rn`]
module"]
#[doc(alias = "ESRH_RN")]
pub type EsrhRn = crate::Reg<esrh_rn::EsrhRnSpec>;
#[doc = "Event Set Register (High Part) CPU write of '1' to the ESRH.En bit causes the ERH.En bit to be set. CPU write of '0' has no effect."]
pub mod esrh_rn;
#[doc = "CER_RN (rw) register accessor: Chained Event Register: If CER.En bit is set (regardless of state of EER.En) then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. CER.En bit is set when a chaining completion code is returned from one of the 3PTCs via the completion interface or is generated internally via Early Completion path. CER.En bit is cleared when the corresponding event is prioritized and serviced. If the CER.En bit is already set and the corresponding chaining completion code is returned from the TC then the corresponding bit in the Event Missed Register is set. CER.En cannot be set or cleared via software.\n\nYou can [`read`](crate::Reg::read) this register and get [`cer_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cer_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cer_rn`]
module"]
#[doc(alias = "CER_RN")]
pub type CerRn = crate::Reg<cer_rn::CerRnSpec>;
#[doc = "Chained Event Register: If CER.En bit is set (regardless of state of EER.En) then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. CER.En bit is set when a chaining completion code is returned from one of the 3PTCs via the completion interface or is generated internally via Early Completion path. CER.En bit is cleared when the corresponding event is prioritized and serviced. If the CER.En bit is already set and the corresponding chaining completion code is returned from the TC then the corresponding bit in the Event Missed Register is set. CER.En cannot be set or cleared via software."]
pub mod cer_rn;
#[doc = "CERH_RN (rw) register accessor: Chained Event Register (High Part): If CERH.En bit is set (regardless of state of EERH.En) then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. CERH.En bit is set when a chaining completion code is returned from one of the 3PTCs via the completion interface or is generated internally via Early Completion path. CERH.En bit is cleared when the corresponding event is prioritized and serviced. If the CERH.En bit is already set and the corresponding chaining completion code is returned from the TC then the corresponding bit in the Event Missed Register is set. CERH.En cannot be set or cleared via software.\n\nYou can [`read`](crate::Reg::read) this register and get [`cerh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cerh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cerh_rn`]
module"]
#[doc(alias = "CERH_RN")]
pub type CerhRn = crate::Reg<cerh_rn::CerhRnSpec>;
#[doc = "Chained Event Register (High Part): If CERH.En bit is set (regardless of state of EERH.En) then the corresponding DMA channel is prioritized vs. other pending DMA events for submission to the TC. CERH.En bit is set when a chaining completion code is returned from one of the 3PTCs via the completion interface or is generated internally via Early Completion path. CERH.En bit is cleared when the corresponding event is prioritized and serviced. If the CERH.En bit is already set and the corresponding chaining completion code is returned from the TC then the corresponding bit in the Event Missed Register is set. CERH.En cannot be set or cleared via software."]
pub mod cerh_rn;
#[doc = "EER_RN (rw) register accessor: Event Enable Register: Enables DMA transfers for ER.En pending events. ER.En is set based on externally asserted events (via tpcc_eventN_pi). This register has no effect on Chained Event Register (CER) or Event Set Register (ESR). Note that if a bit is set in ER.En while EER.En is disabled no action is taken. If EER.En is enabled at a later point (and ER.En has not been cleared via SW) then the event will be recognized as a valid 'TR Sync' EER.En is not directly writeable. Events can be enabled via writes to EESR and can be disabled via writes to EECR register. EER.En = 0: ER.En is not enabled to trigger DMA transfers. EER.En = 1: ER.En is enabled to trigger DMA transfers.\n\nYou can [`read`](crate::Reg::read) this register and get [`eer_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eer_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eer_rn`]
module"]
#[doc(alias = "EER_RN")]
pub type EerRn = crate::Reg<eer_rn::EerRnSpec>;
#[doc = "Event Enable Register: Enables DMA transfers for ER.En pending events. ER.En is set based on externally asserted events (via tpcc_eventN_pi). This register has no effect on Chained Event Register (CER) or Event Set Register (ESR). Note that if a bit is set in ER.En while EER.En is disabled no action is taken. If EER.En is enabled at a later point (and ER.En has not been cleared via SW) then the event will be recognized as a valid 'TR Sync' EER.En is not directly writeable. Events can be enabled via writes to EESR and can be disabled via writes to EECR register. EER.En = 0: ER.En is not enabled to trigger DMA transfers. EER.En = 1: ER.En is enabled to trigger DMA transfers."]
pub mod eer_rn;
#[doc = "EERH_RN (rw) register accessor: Event Enable Register (High Part): Enables DMA transfers for ERH.En pending events. ERH.En is set based on externally asserted events (via tpcc_eventN_pi). This register has no effect on Chained Event Register (CERH) or Event Set Register (ESRH). Note that if a bit is set in ERH.En while EERH.En is disabled no action is taken. If EERH.En is enabled at a later point (and ERH.En has not been cleared via SW) then the event will be recognized as a valid 'TR Sync' EERH.En is not directly writeable. Events can be enabled via writes to EESRH and can be disabled via writes to EECRH register. EERH.En = 0: ER.En is not enabled to trigger DMA transfers. EERH.En = 1: ER.En is enabled to trigger DMA transfers.\n\nYou can [`read`](crate::Reg::read) this register and get [`eerh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eerh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eerh_rn`]
module"]
#[doc(alias = "EERH_RN")]
pub type EerhRn = crate::Reg<eerh_rn::EerhRnSpec>;
#[doc = "Event Enable Register (High Part): Enables DMA transfers for ERH.En pending events. ERH.En is set based on externally asserted events (via tpcc_eventN_pi). This register has no effect on Chained Event Register (CERH) or Event Set Register (ESRH). Note that if a bit is set in ERH.En while EERH.En is disabled no action is taken. If EERH.En is enabled at a later point (and ERH.En has not been cleared via SW) then the event will be recognized as a valid 'TR Sync' EERH.En is not directly writeable. Events can be enabled via writes to EESRH and can be disabled via writes to EECRH register. EERH.En = 0: ER.En is not enabled to trigger DMA transfers. EERH.En = 1: ER.En is enabled to trigger DMA transfers."]
pub mod eerh_rn;
#[doc = "EECR_RN (rw) register accessor: Event Enable Clear Register: CPU write of '1' to the EECR.En bit causes the EER.En bit to be cleared. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`eecr_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecr_rn`]
module"]
#[doc(alias = "EECR_RN")]
pub type EecrRn = crate::Reg<eecr_rn::EecrRnSpec>;
#[doc = "Event Enable Clear Register: CPU write of '1' to the EECR.En bit causes the EER.En bit to be cleared. CPU write of '0' has no effect.."]
pub mod eecr_rn;
#[doc = "EECRH_RN (rw) register accessor: Event Enable Clear Register (High Part): CPU write of '1' to the EECRH.En bit causes the EERH.En bit to be cleared. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`eecrh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecrh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecrh_rn`]
module"]
#[doc(alias = "EECRH_RN")]
pub type EecrhRn = crate::Reg<eecrh_rn::EecrhRnSpec>;
#[doc = "Event Enable Clear Register (High Part): CPU write of '1' to the EECRH.En bit causes the EERH.En bit to be cleared. CPU write of '0' has no effect.."]
pub mod eecrh_rn;
#[doc = "EESR_RN (rw) register accessor: Event Enable Set Register: CPU write of '1' to the EESR.En bit causes the EER.En bit to be set. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`eesr_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesr_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eesr_rn`]
module"]
#[doc(alias = "EESR_RN")]
pub type EesrRn = crate::Reg<eesr_rn::EesrRnSpec>;
#[doc = "Event Enable Set Register: CPU write of '1' to the EESR.En bit causes the EER.En bit to be set. CPU write of '0' has no effect.."]
pub mod eesr_rn;
#[doc = "EESRH_RN (rw) register accessor: Event Enable Set Register (High Part): CPU write of '1' to the EESRH.En bit causes the EERH.En bit to be set. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`eesrh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesrh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eesrh_rn`]
module"]
#[doc(alias = "EESRH_RN")]
pub type EesrhRn = crate::Reg<eesrh_rn::EesrhRnSpec>;
#[doc = "Event Enable Set Register (High Part): CPU write of '1' to the EESRH.En bit causes the EERH.En bit to be set. CPU write of '0' has no effect.."]
pub mod eesrh_rn;
#[doc = "SER_RN (rw) register accessor: Secondary Event Register: The secondary event register is used along with the Event Register (ER) to provide information on the state of an Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events.\n\nYou can [`read`](crate::Reg::read) this register and get [`ser_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ser_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ser_rn`]
module"]
#[doc(alias = "SER_RN")]
pub type SerRn = crate::Reg<ser_rn::SerRnSpec>;
#[doc = "Secondary Event Register: The secondary event register is used along with the Event Register (ER) to provide information on the state of an Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events."]
pub mod ser_rn;
#[doc = "SERH_RN (rw) register accessor: Secondary Event Register (High Part): The secondary event register is used along with the Event Register (ERH) to provide information on the state of an Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events.\n\nYou can [`read`](crate::Reg::read) this register and get [`serh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`serh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@serh_rn`]
module"]
#[doc(alias = "SERH_RN")]
pub type SerhRn = crate::Reg<serh_rn::SerhRnSpec>;
#[doc = "Secondary Event Register (High Part): The secondary event register is used along with the Event Register (ERH) to provide information on the state of an Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events."]
pub mod serh_rn;
#[doc = "SECR_RN (rw) register accessor: Secondary Event Clear Register: The secondary event clear register is used to clear the status of the SER registers. CPU write of '1' to the SECR.En bit clears the SER register. CPU write of '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`secr_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secr_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secr_rn`]
module"]
#[doc(alias = "SECR_RN")]
pub type SecrRn = crate::Reg<secr_rn::SecrRnSpec>;
#[doc = "Secondary Event Clear Register: The secondary event clear register is used to clear the status of the SER registers. CPU write of '1' to the SECR.En bit clears the SER register. CPU write of '0' has no effect."]
pub mod secr_rn;
#[doc = "SECRH_RN (rw) register accessor: Secondary Event Clear Register (High Part): The secondary event clear register is used to clear the status of the SERH registers. CPU write of '1' to the SECRH.En bit clears the SERH register. CPU write of '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`secrh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secrh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secrh_rn`]
module"]
#[doc(alias = "SECRH_RN")]
pub type SecrhRn = crate::Reg<secrh_rn::SecrhRnSpec>;
#[doc = "Secondary Event Clear Register (High Part): The secondary event clear register is used to clear the status of the SERH registers. CPU write of '1' to the SECRH.En bit clears the SERH register. CPU write of '0' has no effect."]
pub mod secrh_rn;
#[doc = "IER_RN (rw) register accessor: Int Enable Register: IER.In is not directly writeable. Interrupts can be enabled via writes to IESR and can be disabled via writes to IECR register. IER.In = 0: IPR.In is NOT enabled for interrupts. IER.In = 1: IPR.In IS enabled for interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`ier_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier_rn`]
module"]
#[doc(alias = "IER_RN")]
pub type IerRn = crate::Reg<ier_rn::IerRnSpec>;
#[doc = "Int Enable Register: IER.In is not directly writeable. Interrupts can be enabled via writes to IESR and can be disabled via writes to IECR register. IER.In = 0: IPR.In is NOT enabled for interrupts. IER.In = 1: IPR.In IS enabled for interrupts."]
pub mod ier_rn;
#[doc = "IERH_RN (rw) register accessor: Int Enable Register (High Part): IERH.In is not directly writeable. Interrupts can be enabled via writes to IESRH and can be disabled via writes to IECRH register. IERH.In = 0: IPRH.In is NOT enabled for interrupts. IERH.In = 1: IPRH.In IS enabled for interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`ierh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ierh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ierh_rn`]
module"]
#[doc(alias = "IERH_RN")]
pub type IerhRn = crate::Reg<ierh_rn::IerhRnSpec>;
#[doc = "Int Enable Register (High Part): IERH.In is not directly writeable. Interrupts can be enabled via writes to IESRH and can be disabled via writes to IECRH register. IERH.In = 0: IPRH.In is NOT enabled for interrupts. IERH.In = 1: IPRH.In IS enabled for interrupts."]
pub mod ierh_rn;
#[doc = "IECR_RN (rw) register accessor: Int Enable Clear Register: CPU write of '1' to the IECR.In bit causes the IER.In bit to be cleared. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`iecr_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iecr_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iecr_rn`]
module"]
#[doc(alias = "IECR_RN")]
pub type IecrRn = crate::Reg<iecr_rn::IecrRnSpec>;
#[doc = "Int Enable Clear Register: CPU write of '1' to the IECR.In bit causes the IER.In bit to be cleared. CPU write of '0' has no effect.."]
pub mod iecr_rn;
#[doc = "IECRH_RN (rw) register accessor: Int Enable Clear Register (High Part): CPU write of '1' to the IECRH.In bit causes the IERH.In bit to be cleared. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`iecrh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iecrh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iecrh_rn`]
module"]
#[doc(alias = "IECRH_RN")]
pub type IecrhRn = crate::Reg<iecrh_rn::IecrhRnSpec>;
#[doc = "Int Enable Clear Register (High Part): CPU write of '1' to the IECRH.In bit causes the IERH.In bit to be cleared. CPU write of '0' has no effect.."]
pub mod iecrh_rn;
#[doc = "IESR_RN (rw) register accessor: Int Enable Set Register: CPU write of '1' to the IESR.In bit causes the IESR.In bit to be set. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`iesr_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iesr_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iesr_rn`]
module"]
#[doc(alias = "IESR_RN")]
pub type IesrRn = crate::Reg<iesr_rn::IesrRnSpec>;
#[doc = "Int Enable Set Register: CPU write of '1' to the IESR.In bit causes the IESR.In bit to be set. CPU write of '0' has no effect.."]
pub mod iesr_rn;
#[doc = "IESRH_RN (rw) register accessor: Int Enable Set Register (High Part): CPU write of '1' to the IESRH.In bit causes the IESRH.In bit to be set. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`iesrh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iesrh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iesrh_rn`]
module"]
#[doc(alias = "IESRH_RN")]
pub type IesrhRn = crate::Reg<iesrh_rn::IesrhRnSpec>;
#[doc = "Int Enable Set Register (High Part): CPU write of '1' to the IESRH.In bit causes the IESRH.In bit to be set. CPU write of '0' has no effect.."]
pub mod iesrh_rn;
#[doc = "IPR_RN (rw) register accessor: Interrupt Pending Register: IPR.In bit is set when a interrupt completion code with TCC of N is detected. IPR.In bit is cleared via software by writing a '1' to ICR.In bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipr_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipr_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr_rn`]
module"]
#[doc(alias = "IPR_RN")]
pub type IprRn = crate::Reg<ipr_rn::IprRnSpec>;
#[doc = "Interrupt Pending Register: IPR.In bit is set when a interrupt completion code with TCC of N is detected. IPR.In bit is cleared via software by writing a '1' to ICR.In bit."]
pub mod ipr_rn;
#[doc = "IPRH_RN (rw) register accessor: Interrupt Pending Register (High Part): IPRH.In bit is set when a interrupt completion code with TCC of N is detected. IPRH.In bit is cleared via software by writing a '1' to ICRH.In bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`iprh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iprh_rn`]
module"]
#[doc(alias = "IPRH_RN")]
pub type IprhRn = crate::Reg<iprh_rn::IprhRnSpec>;
#[doc = "Interrupt Pending Register (High Part): IPRH.In bit is set when a interrupt completion code with TCC of N is detected. IPRH.In bit is cleared via software by writing a '1' to ICRH.In bit."]
pub mod iprh_rn;
#[doc = "ICR_RN (rw) register accessor: Interrupt Clear Register: CPU write of '1' to the ICR.In bit causes the IPR.In bit to be cleared. CPU write of '0' has no effect. All IPR.In bits must be cleared before additional interrupts will be asserted by CC.\n\nYou can [`read`](crate::Reg::read) this register and get [`icr_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr_rn`]
module"]
#[doc(alias = "ICR_RN")]
pub type IcrRn = crate::Reg<icr_rn::IcrRnSpec>;
#[doc = "Interrupt Clear Register: CPU write of '1' to the ICR.In bit causes the IPR.In bit to be cleared. CPU write of '0' has no effect. All IPR.In bits must be cleared before additional interrupts will be asserted by CC."]
pub mod icr_rn;
#[doc = "ICRH_RN (rw) register accessor: Interrupt Clear Register (High Part): CPU write of '1' to the ICRH.In bit causes the IPRH.In bit to be cleared. CPU write of '0' has no effect. All IPRH.In bits must be cleared before additional interrupts will be asserted by CC.\n\nYou can [`read`](crate::Reg::read) this register and get [`icrh_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icrh_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icrh_rn`]
module"]
#[doc(alias = "ICRH_RN")]
pub type IcrhRn = crate::Reg<icrh_rn::IcrhRnSpec>;
#[doc = "Interrupt Clear Register (High Part): CPU write of '1' to the ICRH.In bit causes the IPRH.In bit to be cleared. CPU write of '0' has no effect. All IPRH.In bits must be cleared before additional interrupts will be asserted by CC."]
pub mod icrh_rn;
#[doc = "IEVAL_RN (rw) register accessor: Interrupt Eval Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ieval_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ieval_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ieval_rn`]
module"]
#[doc(alias = "IEVAL_RN")]
pub type IevalRn = crate::Reg<ieval_rn::IevalRnSpec>;
#[doc = "Interrupt Eval Register"]
pub mod ieval_rn;
#[doc = "QER_RN (rw) register accessor: QDMA Event Register: If QER.En bit is set then the corresponding QDMA channel is prioritized vs. other qdma events for submission to the TC. QER.En bit is set when a vbus write byte matches the address defined in the QCHMAPn register. QER.En bit is cleared when the corresponding event is prioritized and serviced. QER.En is also cleared when user writes a '1' to the QSECR.En bit. If the QER.En bit is already set and a new QDMA event is detected due to user write to QDMA trigger location and QEER register is set then the corresponding bit in the QDMA Event Missed Register is set.\n\nYou can [`read`](crate::Reg::read) this register and get [`qer_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qer_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qer_rn`]
module"]
#[doc(alias = "QER_RN")]
pub type QerRn = crate::Reg<qer_rn::QerRnSpec>;
#[doc = "QDMA Event Register: If QER.En bit is set then the corresponding QDMA channel is prioritized vs. other qdma events for submission to the TC. QER.En bit is set when a vbus write byte matches the address defined in the QCHMAPn register. QER.En bit is cleared when the corresponding event is prioritized and serviced. QER.En is also cleared when user writes a '1' to the QSECR.En bit. If the QER.En bit is already set and a new QDMA event is detected due to user write to QDMA trigger location and QEER register is set then the corresponding bit in the QDMA Event Missed Register is set."]
pub mod qer_rn;
#[doc = "QEER_RN (rw) register accessor: QDMA Event Enable Register: Enabled/disabled QDMA address comparator for QDMA Channel N. QEER.En is not directly writeable. QDMA channels can be enabled via writes to QEESR and can be disabled via writes to QEECR register. QEER.En = 1 The corresponding QDMA channel comparator is enabled and Events will be recognized and latched in QER.En. QEER.En = 0 The corresponding QDMA channel comparator is disabled. Events will not be recognized/latched in QER.En.\n\nYou can [`read`](crate::Reg::read) this register and get [`qeer_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qeer_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qeer_rn`]
module"]
#[doc(alias = "QEER_RN")]
pub type QeerRn = crate::Reg<qeer_rn::QeerRnSpec>;
#[doc = "QDMA Event Enable Register: Enabled/disabled QDMA address comparator for QDMA Channel N. QEER.En is not directly writeable. QDMA channels can be enabled via writes to QEESR and can be disabled via writes to QEECR register. QEER.En = 1 The corresponding QDMA channel comparator is enabled and Events will be recognized and latched in QER.En. QEER.En = 0 The corresponding QDMA channel comparator is disabled. Events will not be recognized/latched in QER.En."]
pub mod qeer_rn;
#[doc = "QEECR_RN (rw) register accessor: QDMA Event Enable Clear Register: CPU write of '1' to the QEECR.En bit causes the QEER.En bit to be cleared. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`qeecr_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qeecr_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qeecr_rn`]
module"]
#[doc(alias = "QEECR_RN")]
pub type QeecrRn = crate::Reg<qeecr_rn::QeecrRnSpec>;
#[doc = "QDMA Event Enable Clear Register: CPU write of '1' to the QEECR.En bit causes the QEER.En bit to be cleared. CPU write of '0' has no effect.."]
pub mod qeecr_rn;
#[doc = "QEESR_RN (rw) register accessor: QDMA Event Enable Set Register: CPU write of '1' to the QEESR.En bit causes the QEESR.En bit to be set. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`qeesr_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qeesr_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qeesr_rn`]
module"]
#[doc(alias = "QEESR_RN")]
pub type QeesrRn = crate::Reg<qeesr_rn::QeesrRnSpec>;
#[doc = "QDMA Event Enable Set Register: CPU write of '1' to the QEESR.En bit causes the QEESR.En bit to be set. CPU write of '0' has no effect.."]
pub mod qeesr_rn;
#[doc = "QSER_RN (rw) register accessor: QDMA Secondary Event Register: The QDMA secondary event register is used along with the QDMA Event Register (QER) to provide information on the state of a QDMA Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events.\n\nYou can [`read`](crate::Reg::read) this register and get [`qser_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qser_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qser_rn`]
module"]
#[doc(alias = "QSER_RN")]
pub type QserRn = crate::Reg<qser_rn::QserRnSpec>;
#[doc = "QDMA Secondary Event Register: The QDMA secondary event register is used along with the QDMA Event Register (QER) to provide information on the state of a QDMA Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events."]
pub mod qser_rn;
#[doc = "QSECR_RN (rw) register accessor: QDMA Secondary Event Clear Register: The secondary event clear register is used to clear the status of the QSER and QER register (note that this is slightly different than the SER operation which does not clear the ER.En register). CPU write of '1' to the QSECR.En bit clears the QSER.En and QER.En register fields. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`qsecr_rn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsecr_rn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qsecr_rn`]
module"]
#[doc(alias = "QSECR_RN")]
pub type QsecrRn = crate::Reg<qsecr_rn::QsecrRnSpec>;
#[doc = "QDMA Secondary Event Clear Register: The secondary event clear register is used to clear the status of the QSER and QER register (note that this is slightly different than the SER operation which does not clear the ER.En register). CPU write of '1' to the QSECR.En bit clears the QSER.En and QER.En register fields. CPU write of '0' has no effect.."]
pub mod qsecr_rn;
#[doc = "OPT (rw) register accessor: Options Parameter\n\nYou can [`read`](crate::Reg::read) this register and get [`opt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opt`]
module"]
#[doc(alias = "OPT")]
pub type Opt = crate::Reg<opt::OptSpec>;
#[doc = "Options Parameter"]
pub mod opt;
#[doc = "SRC (rw) register accessor: Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src`]
module"]
#[doc(alias = "SRC")]
pub type Src = crate::Reg<src::SrcSpec>;
#[doc = "Source Address"]
pub mod src;
#[doc = "ABCNT (rw) register accessor: A and B byte count\n\nYou can [`read`](crate::Reg::read) this register and get [`abcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abcnt`]
module"]
#[doc(alias = "ABCNT")]
pub type Abcnt = crate::Reg<abcnt::AbcntSpec>;
#[doc = "A and B byte count"]
pub mod abcnt;
#[doc = "DST (rw) register accessor: Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst`]
module"]
#[doc(alias = "DST")]
pub type Dst = crate::Reg<dst::DstSpec>;
#[doc = "Destination Address"]
pub mod dst;
#[doc = "BIDX (rw) register accessor: Register description is not available\n\nYou can [`read`](crate::Reg::read) this register and get [`bidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bidx`]
module"]
#[doc(alias = "BIDX")]
pub type Bidx = crate::Reg<bidx::BidxSpec>;
#[doc = "Register description is not available"]
pub mod bidx;
#[doc = "LNK (rw) register accessor: Link and Reload parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`lnk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lnk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lnk`]
module"]
#[doc(alias = "LNK")]
pub type Lnk = crate::Reg<lnk::LnkSpec>;
#[doc = "Link and Reload parameters"]
pub mod lnk;
#[doc = "CIDX (rw) register accessor: Register description is not available\n\nYou can [`read`](crate::Reg::read) this register and get [`cidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidx`]
module"]
#[doc(alias = "CIDX")]
pub type Cidx = crate::Reg<cidx::CidxSpec>;
#[doc = "Register description is not available"]
pub mod cidx;
#[doc = "CCNT (rw) register accessor: C byte count\n\nYou can [`read`](crate::Reg::read) this register and get [`ccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt`]
module"]
#[doc(alias = "CCNT")]
pub type Ccnt = crate::Reg<ccnt::CcntSpec>;
#[doc = "C byte count"]
pub mod ccnt;
