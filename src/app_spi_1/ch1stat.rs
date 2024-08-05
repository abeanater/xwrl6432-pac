#[doc = "Register `CH1STAT` reader"]
pub type R = crate::R<Ch1statSpec>;
#[doc = "Register `CH1STAT` writer"]
pub type W = crate::W<Ch1statSpec>;
#[doc = "Field `RXS` reader - 0:0\\]
Channel \"i\" Receiver Register Status - (RO )"]
pub type RxsR = crate::BitReader;
#[doc = "Field `RXS` writer - 0:0\\]
Channel \"i\" Receiver Register Status - (RO )"]
pub type RxsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXS` reader - 1:1\\]
Channel \"i\" Transmitter Register Status - (RO )"]
pub type TxsR = crate::BitReader;
#[doc = "Field `TXS` writer - 1:1\\]
Channel \"i\" Transmitter Register Status - (RO )"]
pub type TxsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOT` reader - 2:2\\]
Channel \"i\" End of transfer Status The definitions of beginning and end of transfer vary with master versus slave and the transfer format \\[Transmit/Receive modes Turbo mode\\]
See dedicated chapters for details - (RO )"]
pub type EotR = crate::BitReader;
#[doc = "Field `EOT` writer - 2:2\\]
Channel \"i\" End of transfer Status The definitions of beginning and end of transfer vary with master versus slave and the transfer format \\[Transmit/Receive modes Turbo mode\\]
See dedicated chapters for details - (RO )"]
pub type EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFE` reader - 3:3\\]
Channel \"i\" FIFO Transmit Buffer Empty Status - (RO )"]
pub type TxffeR = crate::BitReader;
#[doc = "Field `TXFFE` writer - 3:3\\]
Channel \"i\" FIFO Transmit Buffer Empty Status - (RO )"]
pub type TxffeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFF` reader - 4:4\\]
Channel \"i\" FIFO Transmit Buffer Full Status - (RO )"]
pub type TxfffR = crate::BitReader;
#[doc = "Field `TXFFF` writer - 4:4\\]
Channel \"i\" FIFO Transmit Buffer Full Status - (RO )"]
pub type TxfffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFE` reader - 5:5\\]
Channel \"i\" FIFO Receive Buffer Empty Status - (RO )"]
pub type RxffeR = crate::BitReader;
#[doc = "Field `RXFFE` writer - 5:5\\]
Channel \"i\" FIFO Receive Buffer Empty Status - (RO )"]
pub type RxffeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFF` reader - 6:6\\]
Channel \"i\" FIFO Receive Buffer Full Status - (RO )"]
pub type RxfffR = crate::BitReader;
#[doc = "Field `RXFFF` writer - 6:6\\]
Channel \"i\" FIFO Receive Buffer Full Status - (RO )"]
pub type RxfffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED_2` reader - 31:7\\]
Read returns 0 - (RO )"]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_2` writer - 31:7\\]
Read returns 0 - (RO )"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel \"i\" Receiver Register Status - (RO )"]
    #[inline(always)]
    pub fn rxs(&self) -> RxsR {
        RxsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel \"i\" Transmitter Register Status - (RO )"]
    #[inline(always)]
    pub fn txs(&self) -> TxsR {
        TxsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel \"i\" End of transfer Status The definitions of beginning and end of transfer vary with master versus slave and the transfer format \\[Transmit/Receive modes Turbo mode\\]
See dedicated chapters for details - (RO )"]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel \"i\" FIFO Transmit Buffer Empty Status - (RO )"]
    #[inline(always)]
    pub fn txffe(&self) -> TxffeR {
        TxffeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel \"i\" FIFO Transmit Buffer Full Status - (RO )"]
    #[inline(always)]
    pub fn txfff(&self) -> TxfffR {
        TxfffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Channel \"i\" FIFO Receive Buffer Empty Status - (RO )"]
    #[inline(always)]
    pub fn rxffe(&self) -> RxffeR {
        RxffeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Channel \"i\" FIFO Receive Buffer Full Status - (RO )"]
    #[inline(always)]
    pub fn rxfff(&self) -> RxfffR {
        RxfffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Read returns 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel \"i\" Receiver Register Status - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn rxs(&mut self) -> RxsW<Ch1statSpec> {
        RxsW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel \"i\" Transmitter Register Status - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn txs(&mut self) -> TxsW<Ch1statSpec> {
        TxsW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel \"i\" End of transfer Status The definitions of beginning and end of transfer vary with master versus slave and the transfer format \\[Transmit/Receive modes Turbo mode\\]
See dedicated chapters for details - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn eot(&mut self) -> EotW<Ch1statSpec> {
        EotW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel \"i\" FIFO Transmit Buffer Empty Status - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn txffe(&mut self) -> TxffeW<Ch1statSpec> {
        TxffeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel \"i\" FIFO Transmit Buffer Full Status - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn txfff(&mut self) -> TxfffW<Ch1statSpec> {
        TxfffW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Channel \"i\" FIFO Receive Buffer Empty Status - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn rxffe(&mut self) -> RxffeW<Ch1statSpec> {
        RxffeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Channel \"i\" FIFO Receive Buffer Full Status - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn rxfff(&mut self) -> RxfffW<Ch1statSpec> {
        RxfffW::new(self, 6)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Read returns 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_2(&mut self) -> Reserved2W<Ch1statSpec> {
        Reserved2W::new(self, 7)
    }
}
#[doc = "This register provides status information about transmitter and receiver registers of channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1statSpec;
impl crate::RegisterSpec for Ch1statSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1stat::R`](R) reader structure"]
impl crate::Readable for Ch1statSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1stat::W`](W) writer structure"]
impl crate::Writable for Ch1statSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1STAT to value 0"]
impl crate::Resettable for Ch1statSpec {
    const RESET_VALUE: u32 = 0;
}
