#[doc = "Register `XFERLEVEL` reader"]
pub type R = crate::R<XferlevelSpec>;
#[doc = "Register `XFERLEVEL` writer"]
pub type W = crate::W<XferlevelSpec>;
#[doc = "Field `AEL` reader - 7:0\\]
Buffer Almost EmptyThis register holds the programmable almost empty level value used to determine almost empty buffer condition If the user wants an interrupt or a DMA write request to be issued during a transmit operation when the data buffer is able to receive n bytes then the buffer MCSPI_MODULCTRL\\[AEL\\]
must be set with n-1 - (RW )"]
pub type AelR = crate::FieldReader;
#[doc = "Field `AEL` writer - 7:0\\]
Buffer Almost EmptyThis register holds the programmable almost empty level value used to determine almost empty buffer condition If the user wants an interrupt or a DMA write request to be issued during a transmit operation when the data buffer is able to receive n bytes then the buffer MCSPI_MODULCTRL\\[AEL\\]
must be set with n-1 - (RW )"]
pub type AelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AFL` reader - 15:8\\]
Buffer Almost Full This register holds the programmable almost full level value used to determine almost full buffer condition If the user wants an interrupt or a DMA read request to be issued during a receive operation when the data buffer holds at least n bytes then the buffer MCSPI_MODULCTRL\\[AFL\\]
must be set with n-1The size of this register is defined by the generic parameter FFNBYTE - (RW )"]
pub type AflR = crate::FieldReader;
#[doc = "Field `AFL` writer - 15:8\\]
Buffer Almost Full This register holds the programmable almost full level value used to determine almost full buffer condition If the user wants an interrupt or a DMA read request to be issued during a receive operation when the data buffer holds at least n bytes then the buffer MCSPI_MODULCTRL\\[AFL\\]
must be set with n-1The size of this register is defined by the generic parameter FFNBYTE - (RW )"]
pub type AflW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WCNT` reader - 31:16\\]
Spi word counterThis register holds the programmable value of number of SPI word to be transferred on channel which is using the FIFO bufferWhen transfer had started a read back in this register returns the current SPI word transfer index - (RW )"]
pub type WcntR = crate::FieldReader<u16>;
#[doc = "Field `WCNT` writer - 31:16\\]
Spi word counterThis register holds the programmable value of number of SPI word to be transferred on channel which is using the FIFO bufferWhen transfer had started a read back in this register returns the current SPI word transfer index - (RW )"]
pub type WcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Buffer Almost EmptyThis register holds the programmable almost empty level value used to determine almost empty buffer condition If the user wants an interrupt or a DMA write request to be issued during a transmit operation when the data buffer is able to receive n bytes then the buffer MCSPI_MODULCTRL\\[AEL\\]
must be set with n-1 - (RW )"]
    #[inline(always)]
    pub fn ael(&self) -> AelR {
        AelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Buffer Almost Full This register holds the programmable almost full level value used to determine almost full buffer condition If the user wants an interrupt or a DMA read request to be issued during a receive operation when the data buffer holds at least n bytes then the buffer MCSPI_MODULCTRL\\[AFL\\]
must be set with n-1The size of this register is defined by the generic parameter FFNBYTE - (RW )"]
    #[inline(always)]
    pub fn afl(&self) -> AflR {
        AflR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Spi word counterThis register holds the programmable value of number of SPI word to be transferred on channel which is using the FIFO bufferWhen transfer had started a read back in this register returns the current SPI word transfer index - (RW )"]
    #[inline(always)]
    pub fn wcnt(&self) -> WcntR {
        WcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Buffer Almost EmptyThis register holds the programmable almost empty level value used to determine almost empty buffer condition If the user wants an interrupt or a DMA write request to be issued during a transmit operation when the data buffer is able to receive n bytes then the buffer MCSPI_MODULCTRL\\[AEL\\]
must be set with n-1 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn ael(&mut self) -> AelW<XferlevelSpec> {
        AelW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Buffer Almost Full This register holds the programmable almost full level value used to determine almost full buffer condition If the user wants an interrupt or a DMA read request to be issued during a receive operation when the data buffer holds at least n bytes then the buffer MCSPI_MODULCTRL\\[AFL\\]
must be set with n-1The size of this register is defined by the generic parameter FFNBYTE - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn afl(&mut self) -> AflW<XferlevelSpec> {
        AflW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Spi word counterThis register holds the programmable value of number of SPI word to be transferred on channel which is using the FIFO bufferWhen transfer had started a read back in this register returns the current SPI word transfer index - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn wcnt(&mut self) -> WcntW<XferlevelSpec> {
        WcntW::new(self, 16)
    }
}
#[doc = "This register provides transfer levels needed while using FIFO buffer during transfer.\n\nYou can [`read`](crate::Reg::read) this register and get [`xferlevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xferlevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XferlevelSpec;
impl crate::RegisterSpec for XferlevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xferlevel::R`](R) reader structure"]
impl crate::Readable for XferlevelSpec {}
#[doc = "`write(|w| ..)` method takes [`xferlevel::W`](W) writer structure"]
impl crate::Writable for XferlevelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XFERLEVEL to value 0"]
impl crate::Resettable for XferlevelSpec {
    const RESET_VALUE: u32 = 0;
}
