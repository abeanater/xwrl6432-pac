#[doc = "Register `MBRSR` reader"]
pub type R = crate::R<MbrsrSpec>;
#[doc = "Register `MBRSR` writer"]
pub type W = crate::W<MbrsrSpec>;
#[doc = "Field `MBR` reader - 12:0\\]
Maximum Baud Rate Prescaler. This field is effective in LIN mode only. This 13-bit prescaler is used during the synchronization phase (see the \"Header Reception and Adaptive Baudrate\" section) of a slave module if the ADAPT bit is set. In this way, a SCI/LIN slave using an automatic or select bit rate modes detects any LIN bus legal rate automatically. The MBR value should be programmed to allow a maximum baud rate that is not more than 10% above the expected operating baud rate in the LIN network. Otherwise a s 0x00 data byte could mistakenly be detected as sync break. The default value is for a 70MHz LINCLK (0xDAC). This MBR prescaler is used by the wake-up and idle time counters for a constant expiration time relative to a 20kHz rate."]
pub type MbrR = crate::FieldReader<u16>;
#[doc = "Field `MBR` writer - 12:0\\]
Maximum Baud Rate Prescaler. This field is effective in LIN mode only. This 13-bit prescaler is used during the synchronization phase (see the \"Header Reception and Adaptive Baudrate\" section) of a slave module if the ADAPT bit is set. In this way, a SCI/LIN slave using an automatic or select bit rate modes detects any LIN bus legal rate automatically. The MBR value should be programmed to allow a maximum baud rate that is not more than 10% above the expected operating baud rate in the LIN network. Otherwise a s 0x00 data byte could mistakenly be detected as sync break. The default value is for a 70MHz LINCLK (0xDAC). This MBR prescaler is used by the wake-up and idle time counters for a constant expiration time relative to a 20kHz rate."]
pub type MbrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
Maximum Baud Rate Prescaler. This field is effective in LIN mode only. This 13-bit prescaler is used during the synchronization phase (see the \"Header Reception and Adaptive Baudrate\" section) of a slave module if the ADAPT bit is set. In this way, a SCI/LIN slave using an automatic or select bit rate modes detects any LIN bus legal rate automatically. The MBR value should be programmed to allow a maximum baud rate that is not more than 10% above the expected operating baud rate in the LIN network. Otherwise a s 0x00 data byte could mistakenly be detected as sync break. The default value is for a 70MHz LINCLK (0xDAC). This MBR prescaler is used by the wake-up and idle time counters for a constant expiration time relative to a 20kHz rate."]
    #[inline(always)]
    pub fn mbr(&self) -> MbrR {
        MbrR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 12:0\\]
Maximum Baud Rate Prescaler. This field is effective in LIN mode only. This 13-bit prescaler is used during the synchronization phase (see the \"Header Reception and Adaptive Baudrate\" section) of a slave module if the ADAPT bit is set. In this way, a SCI/LIN slave using an automatic or select bit rate modes detects any LIN bus legal rate automatically. The MBR value should be programmed to allow a maximum baud rate that is not more than 10% above the expected operating baud rate in the LIN network. Otherwise a s 0x00 data byte could mistakenly be detected as sync break. The default value is for a 70MHz LINCLK (0xDAC). This MBR prescaler is used by the wake-up and idle time counters for a constant expiration time relative to a 20kHz rate."]
    #[inline(always)]
    #[must_use]
    pub fn mbr(&mut self) -> MbrW<MbrsrSpec> {
        MbrW::new(self, 0)
    }
}
#[doc = "The MBRSR register is used to configure the expected maximum baud rate of the LIN network.\n\nYou can [`read`](crate::Reg::read) this register and get [`mbrsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbrsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbrsrSpec;
impl crate::RegisterSpec for MbrsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbrsr::R`](R) reader structure"]
impl crate::Readable for MbrsrSpec {}
#[doc = "`write(|w| ..)` method takes [`mbrsr::W`](W) writer structure"]
impl crate::Writable for MbrsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MBRSR to value 0"]
impl crate::Resettable for MbrsrSpec {
    const RESET_VALUE: u32 = 0;
}
