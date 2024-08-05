#[doc = "Register `DAFRX` reader"]
pub type R = crate::R<DafrxSpec>;
#[doc = "Register `DAFRX` writer"]
pub type W = crate::W<DafrxSpec>;
#[doc = "Field `DAFRDATA` reader - 31:0\\]
FIFO Data to transmit with DMA 256 bit aligned address This Register is only is used when MCSPI_MODULCTRL\\[FDAA\\]
is set to \"1\" and only one of the MCSPI_CH\\[i\\]CONF\\[FFEW\\]
of enabled channels is set If these conditions are not respected any access to this register return a null value - (RO )"]
pub type DafrdataR = crate::FieldReader<u32>;
#[doc = "Field `DAFRDATA` writer - 31:0\\]
FIFO Data to transmit with DMA 256 bit aligned address This Register is only is used when MCSPI_MODULCTRL\\[FDAA\\]
is set to \"1\" and only one of the MCSPI_CH\\[i\\]CONF\\[FFEW\\]
of enabled channels is set If these conditions are not respected any access to this register return a null value - (RO )"]
pub type DafrdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
FIFO Data to transmit with DMA 256 bit aligned address This Register is only is used when MCSPI_MODULCTRL\\[FDAA\\]
is set to \"1\" and only one of the MCSPI_CH\\[i\\]CONF\\[FFEW\\]
of enabled channels is set If these conditions are not respected any access to this register return a null value - (RO )"]
    #[inline(always)]
    pub fn dafrdata(&self) -> DafrdataR {
        DafrdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
FIFO Data to transmit with DMA 256 bit aligned address This Register is only is used when MCSPI_MODULCTRL\\[FDAA\\]
is set to \"1\" and only one of the MCSPI_CH\\[i\\]CONF\\[FFEW\\]
of enabled channels is set If these conditions are not respected any access to this register return a null value - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn dafrdata(&mut self) -> DafrdataW<DafrxSpec> {
        DafrdataW::new(self, 0)
    }
}
#[doc = "This register contains the SPI words to received on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_RX(i) register corresponding to the channel which have its FIFO enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`dafrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dafrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DafrxSpec;
impl crate::RegisterSpec for DafrxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dafrx::R`](R) reader structure"]
impl crate::Readable for DafrxSpec {}
#[doc = "`write(|w| ..)` method takes [`dafrx::W`](W) writer structure"]
impl crate::Writable for DafrxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAFRX to value 0"]
impl crate::Resettable for DafrxSpec {
    const RESET_VALUE: u32 = 0;
}
