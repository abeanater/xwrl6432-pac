#[doc = "Register `HWACCREG12` reader"]
pub type R = crate::R<Hwaccreg12Spec>;
#[doc = "Register `HWACCREG12` writer"]
pub type W = crate::W<Hwaccreg12Spec>;
#[doc = "Field `ACC_TRIGGER_IN_STAT` reader - 18:0\\]
Debug register for trigger status: This is a read-only status register, which indicates the trigger status of the accelerator, i.e., whether a specific DMA trigger or a Ping-pong trigger or a SW trigger was ever received (refer TRIGMODE in HW_ACC_PARAM register set). The MSB 16 bits of this register indicate whether a trigger was received via DMA trigger method. The next two bits (i.e., bit indices 2 and 1) indicate the status of DFE ping-pong switch-based trigger and SW trigger respectively. The LSB bit is always 1 and can be ignored {DMA2ACCTRIG\\[15:0\\],adc_buffer_done,CM42ACCTRIG,1}"]
pub type AccTriggerInStatR = crate::FieldReader<u32>;
#[doc = "Field `ACC_TRIGGER_IN_STAT` writer - 18:0\\]
Debug register for trigger status: This is a read-only status register, which indicates the trigger status of the accelerator, i.e., whether a specific DMA trigger or a Ping-pong trigger or a SW trigger was ever received (refer TRIGMODE in HW_ACC_PARAM register set). The MSB 16 bits of this register indicate whether a trigger was received via DMA trigger method. The next two bits (i.e., bit indices 2 and 1) indicate the status of DFE ping-pong switch-based trigger and SW trigger respectively. The LSB bit is always 1 and can be ignored {DMA2ACCTRIG\\[15:0\\],adc_buffer_done,CM42ACCTRIG,1}"]
pub type AccTriggerInStatW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ACC_TRIGGER_IN_CLR` reader - 24:24\\]
Clear trigger status read-only register: This register-bit when set clears the trigger status register ACC_TRIG_IN_STAT described above"]
pub type AccTriggerInClrR = crate::BitReader;
#[doc = "Field `ACC_TRIGGER_IN_CLR` writer - 24:24\\]
Clear trigger status read-only register: This register-bit when set clears the trigger status register ACC_TRIG_IN_STAT described above"]
pub type AccTriggerInClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:18 - 18:0\\]
Debug register for trigger status: This is a read-only status register, which indicates the trigger status of the accelerator, i.e., whether a specific DMA trigger or a Ping-pong trigger or a SW trigger was ever received (refer TRIGMODE in HW_ACC_PARAM register set). The MSB 16 bits of this register indicate whether a trigger was received via DMA trigger method. The next two bits (i.e., bit indices 2 and 1) indicate the status of DFE ping-pong switch-based trigger and SW trigger respectively. The LSB bit is always 1 and can be ignored {DMA2ACCTRIG\\[15:0\\],adc_buffer_done,CM42ACCTRIG,1}"]
    #[inline(always)]
    pub fn acc_trigger_in_stat(&self) -> AccTriggerInStatR {
        AccTriggerInStatR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:23"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Clear trigger status read-only register: This register-bit when set clears the trigger status register ACC_TRIG_IN_STAT described above"]
    #[inline(always)]
    pub fn acc_trigger_in_clr(&self) -> AccTriggerInClrR {
        AccTriggerInClrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - 18:0\\]
Debug register for trigger status: This is a read-only status register, which indicates the trigger status of the accelerator, i.e., whether a specific DMA trigger or a Ping-pong trigger or a SW trigger was ever received (refer TRIGMODE in HW_ACC_PARAM register set). The MSB 16 bits of this register indicate whether a trigger was received via DMA trigger method. The next two bits (i.e., bit indices 2 and 1) indicate the status of DFE ping-pong switch-based trigger and SW trigger respectively. The LSB bit is always 1 and can be ignored {DMA2ACCTRIG\\[15:0\\],adc_buffer_done,CM42ACCTRIG,1}"]
    #[inline(always)]
    #[must_use]
    pub fn acc_trigger_in_stat(&mut self) -> AccTriggerInStatW<Hwaccreg12Spec> {
        AccTriggerInStatW::new(self, 0)
    }
    #[doc = "Bits 19:23"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Hwaccreg12Spec> {
        Nu1W::new(self, 19)
    }
    #[doc = "Bit 24 - 24:24\\]
Clear trigger status read-only register: This register-bit when set clears the trigger status register ACC_TRIG_IN_STAT described above"]
    #[inline(always)]
    #[must_use]
    pub fn acc_trigger_in_clr(&mut self) -> AccTriggerInClrW<Hwaccreg12Spec> {
        AccTriggerInClrW::new(self, 24)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Hwaccreg12Spec> {
        Nu2W::new(self, 25)
    }
}
#[doc = "HWACCREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg12Spec;
impl crate::RegisterSpec for Hwaccreg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg12::R`](R) reader structure"]
impl crate::Readable for Hwaccreg12Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg12::W`](W) writer structure"]
impl crate::Writable for Hwaccreg12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG12 to value 0"]
impl crate::Resettable for Hwaccreg12Spec {
    const RESET_VALUE: u32 = 0;
}
