#[doc = "Register `SCIPIO6` reader"]
pub type R = crate::R<Scipio6Spec>;
#[doc = "Register `SCIPIO6` writer"]
pub type W = crate::W<Scipio6Spec>;
#[doc = "Field `CLK_PDR` reader - 0:0\\]
CLK Open Drain Enable Enables open-drain capability in the output pin SCICLK if the following conditions are met: CLK DATA DIR = 1 (SCICLK pin is a general-purpose output.) CLK DOUT = 1"]
pub type ClkPdrR = crate::BitReader;
#[doc = "Field `CLK_PDR` writer - 0:0\\]
CLK Open Drain Enable Enables open-drain capability in the output pin SCICLK if the following conditions are met: CLK DATA DIR = 1 (SCICLK pin is a general-purpose output.) CLK DOUT = 1"]
pub type ClkPdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PDR` reader - 1:1\\]
RX Open Drain Enable Enables open-drain capability in the output pin SCIRX if the following conditions are met: RX DATA DIR = 1 (SCIRX pin is a general-purpose output.) RX DOUT = 1"]
pub type RxPdrR = crate::BitReader;
#[doc = "Field `RX_PDR` writer - 1:1\\]
RX Open Drain Enable Enables open-drain capability in the output pin SCIRX if the following conditions are met: RX DATA DIR = 1 (SCIRX pin is a general-purpose output.) RX DOUT = 1"]
pub type RxPdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PDR` reader - 2:2\\]
TX Open Drain Enable Enables open-drain capability in the output pin SCITX if the following conditions are met: TX DATA DIR = 1 (SCITX pin is a general-purpose output.) TX DOUT = 1"]
pub type TxPdrR = crate::BitReader;
#[doc = "Field `TX_PDR` writer - 2:2\\]
TX Open Drain Enable Enables open-drain capability in the output pin SCITX if the following conditions are met: TX DATA DIR = 1 (SCITX pin is a general-purpose output.) TX DOUT = 1"]
pub type TxPdrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CLK Open Drain Enable Enables open-drain capability in the output pin SCICLK if the following conditions are met: CLK DATA DIR = 1 (SCICLK pin is a general-purpose output.) CLK DOUT = 1"]
    #[inline(always)]
    pub fn clk_pdr(&self) -> ClkPdrR {
        ClkPdrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RX Open Drain Enable Enables open-drain capability in the output pin SCIRX if the following conditions are met: RX DATA DIR = 1 (SCIRX pin is a general-purpose output.) RX DOUT = 1"]
    #[inline(always)]
    pub fn rx_pdr(&self) -> RxPdrR {
        RxPdrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TX Open Drain Enable Enables open-drain capability in the output pin SCITX if the following conditions are met: TX DATA DIR = 1 (SCITX pin is a general-purpose output.) TX DOUT = 1"]
    #[inline(always)]
    pub fn tx_pdr(&self) -> TxPdrR {
        TxPdrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CLK Open Drain Enable Enables open-drain capability in the output pin SCICLK if the following conditions are met: CLK DATA DIR = 1 (SCICLK pin is a general-purpose output.) CLK DOUT = 1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pdr(&mut self) -> ClkPdrW<Scipio6Spec> {
        ClkPdrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RX Open Drain Enable Enables open-drain capability in the output pin SCIRX if the following conditions are met: RX DATA DIR = 1 (SCIRX pin is a general-purpose output.) RX DOUT = 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pdr(&mut self) -> RxPdrW<Scipio6Spec> {
        RxPdrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
TX Open Drain Enable Enables open-drain capability in the output pin SCITX if the following conditions are met: TX DATA DIR = 1 (SCITX pin is a general-purpose output.) TX DOUT = 1"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdr(&mut self) -> TxPdrW<Scipio6Spec> {
        TxPdrW::new(self, 2)
    }
}
#[doc = "SCI Pin I/O Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio6Spec;
impl crate::RegisterSpec for Scipio6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio6::R`](R) reader structure"]
impl crate::Readable for Scipio6Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio6::W`](W) writer structure"]
impl crate::Writable for Scipio6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO6 to value 0"]
impl crate::Resettable for Scipio6Spec {
    const RESET_VALUE: u32 = 0;
}
