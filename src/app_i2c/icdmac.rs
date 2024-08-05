#[doc = "Register `ICDMAC` reader"]
pub type R = crate::R<IcdmacSpec>;
#[doc = "Register `ICDMAC` writer"]
pub type W = crate::W<IcdmacSpec>;
#[doc = "Field `RXDMAEN` reader - 0:0\\]
Receive DMA enable. This bit controls the receive DMA event pin to the system. When this bit is 1 the DMA event is enabled and ICREVT_POR pin is asserted when the DMA transfer is required. When this bit is 0 the ICREVT_POR pin is never asserted. RXDMAEN=0: DMA receive event is disabled. RXDMAEN=1: DMA receive event is enabled. (Default)"]
pub type RxdmaenR = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - 0:0\\]
Receive DMA enable. This bit controls the receive DMA event pin to the system. When this bit is 1 the DMA event is enabled and ICREVT_POR pin is asserted when the DMA transfer is required. When this bit is 0 the ICREVT_POR pin is never asserted. RXDMAEN=0: DMA receive event is disabled. RXDMAEN=1: DMA receive event is enabled. (Default)"]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAEN` reader - 1:1\\]
Transmit DMA enable. This bit controls the receive DMA event pin to the system. When this bit is 1 the DMA event is enabled and ICTEVT_POR pin is asserted when the DMA transfer is required. When this bit is 0 the ICTEVT_POR pin is never asserted. RXDMAEN=0: DMA transmit event is disabled. RXDMAEN=1: DMA transmit event is enabled. (Default)"]
pub type TxdmaenR = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - 1:1\\]
Transmit DMA enable. This bit controls the receive DMA event pin to the system. When this bit is 1 the DMA event is enabled and ICTEVT_POR pin is asserted when the DMA transfer is required. When this bit is 0 the ICTEVT_POR pin is never asserted. RXDMAEN=0: DMA transmit event is disabled. RXDMAEN=1: DMA transmit event is enabled. (Default)"]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:2\\]
Reserved. - (RW )"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:2\\]
Reserved. - (RW )"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive DMA enable. This bit controls the receive DMA event pin to the system. When this bit is 1 the DMA event is enabled and ICREVT_POR pin is asserted when the DMA transfer is required. When this bit is 0 the ICREVT_POR pin is never asserted. RXDMAEN=0: DMA receive event is disabled. RXDMAEN=1: DMA receive event is enabled. (Default)"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit DMA enable. This bit controls the receive DMA event pin to the system. When this bit is 1 the DMA event is enabled and ICTEVT_POR pin is asserted when the DMA transfer is required. When this bit is 0 the ICTEVT_POR pin is never asserted. RXDMAEN=0: DMA transmit event is disabled. RXDMAEN=1: DMA transmit event is enabled. (Default)"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved. - (RW )"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive DMA enable. This bit controls the receive DMA event pin to the system. When this bit is 1 the DMA event is enabled and ICREVT_POR pin is asserted when the DMA transfer is required. When this bit is 0 the ICREVT_POR pin is never asserted. RXDMAEN=0: DMA receive event is disabled. RXDMAEN=1: DMA receive event is enabled. (Default)"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RxdmaenW<IcdmacSpec> {
        RxdmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit DMA enable. This bit controls the receive DMA event pin to the system. When this bit is 1 the DMA event is enabled and ICTEVT_POR pin is asserted when the DMA transfer is required. When this bit is 0 the ICTEVT_POR pin is never asserted. RXDMAEN=0: DMA transmit event is disabled. RXDMAEN=1: DMA transmit event is enabled. (Default)"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TxdmaenW<IcdmacSpec> {
        TxdmaenW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved. - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcdmacSpec> {
        NuW::new(self, 2)
    }
}
#[doc = "I2C DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icdmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icdmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcdmacSpec;
impl crate::RegisterSpec for IcdmacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icdmac::R`](R) reader structure"]
impl crate::Readable for IcdmacSpec {}
#[doc = "`write(|w| ..)` method takes [`icdmac::W`](W) writer structure"]
impl crate::Writable for IcdmacSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICDMAC to value 0"]
impl crate::Resettable for IcdmacSpec {
    const RESET_VALUE: u32 = 0;
}
